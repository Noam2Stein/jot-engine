use std::mem::replace;

use super::*;

pub trait Chunk2D {
    type Context<'a>;

    fn load(chunk_pos: IVec2, ctx: &mut Self::Context<'_>) -> Self;

    fn unload(self, ctx: &mut Self::Context<'_>);
}

pub struct ChunkHolder2D<
    T: Chunk2D,
    const CHUNK_WIDTH: u32,
    const CHUNK_HEIGHT: u32,
    const CHUNKS_X: usize,
    const CHUNKS_Y: usize,
> {
    chunks: [[Suspendable<T>; CHUNKS_X]; CHUNKS_Y],
    min_chunk_pos: IVec2,
}

impl<
    T: Chunk2D,
    const CHUNK_WIDTH: u32,
    const CHUNK_HEIGHT: u32,
    const CHUNKS_X: usize,
    const CHUNKS_Y: usize,
> ChunkHolder2D<T, CHUNK_WIDTH, CHUNK_HEIGHT, CHUNKS_X, CHUNKS_Y>
{
    pub fn new<'ctx>(pos: SVec2, mut ctx: T::Context<'ctx>) -> Self {
        let min_chunk_pos = Self::get_min_chunk_pos(pos);

        let chunks = std::array::from_fn(|y| {
            std::array::from_fn(|x| {
                Suspendable::Some(T::load(min_chunk_pos + vec2!(x as i32, y as i32), &mut ctx))
            })
        });

        Self {
            chunks,
            min_chunk_pos,
        }
    }

    pub fn target_moved(&mut self, pos: SVec2, mut ctx: T::Context<'_>) {
        let min_chunk_pos = Self::get_min_chunk_pos(pos);

        if min_chunk_pos != self.min_chunk_pos {
            let old_min_chunk_pos = self.min_chunk_pos;
            let mut old_chunks = replace(
                &mut self.chunks,
                std::array::from_fn(|_| std::array::from_fn(|_| Suspendable::Placeholder)),
            );

            self.min_chunk_pos = min_chunk_pos;
            for y in 0..CHUNKS_Y {
                for x in 0..CHUNKS_X {
                    let chunk_pos_in_old =
                        vec2!(x as i32, y as i32) + min_chunk_pos - old_min_chunk_pos;

                    let reuse_x =
                        chunk_pos_in_old.x() >= 0 && chunk_pos_in_old.x() < CHUNKS_X as i32;
                    let reuse_y =
                        chunk_pos_in_old.y() >= 0 && chunk_pos_in_old.y() < CHUNKS_Y as i32;

                    self.chunks[y][x] = if reuse_x && reuse_y {
                        let x_in_old = chunk_pos_in_old.x() as usize;
                        let y_in_old = chunk_pos_in_old.y() as usize;

                        replace(
                            &mut old_chunks[y_in_old][x_in_old],
                            Suspendable::Placeholder,
                        )
                    } else {
                        let chunk_pos = min_chunk_pos + vec2!(x as i32, y as i32);

                        Suspendable::Some(T::load(chunk_pos, &mut ctx))
                    };
                }
            }

            for old_chunk in old_chunks.into_iter().flatten() {
                if let Suspendable::Some(chunk) = old_chunk {
                    chunk.unload(&mut ctx);
                }
            }
        }
    }

    fn get_min_chunk_pos(pos: SVec2) -> IVec2 {
        let chunk_pos = (pos / Self::CHUNK_SIZE).floor().map(s32::as_i32);

        let min_chunk_pos = chunk_pos - Self::CENTER_CHUNK_OFFSET;

        min_chunk_pos
    }

    const CHUNK_SIZE: SVec2 = vec2!(s32::from_u32(CHUNK_WIDTH), s32::from_u32(CHUNK_HEIGHT));
    const CENTER_CHUNK_OFFSET: IVec2 = vec2!((CHUNKS_X as i32 - 1) / 2, (CHUNKS_Y as i32 - 1) / 2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Suspendable<T> {
    Some(T),
    Placeholder,
}
