use std::mem::replace;

use super::*;

pub trait Chunk2D: Sized + 'static {
    type Context<'a>;

    fn load(chunk_pos: IVec2, _offset_from_center: IVec2, ctx: &mut Self::Context<'_>) -> Self;

    fn moved(&mut self, offset_from_center: IVec2, ctx: &mut Self::Context<'_>) {
        let _ = offset_from_center;
        let _ = ctx;
    }

    fn unload(self, ctx: &mut Self::Context<'_>);
}

#[derive(Debug, Clone)]
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
    pub fn new(pos: SVec2, mut ctx: T::Context<'_>) -> Self {
        let min_chunk_pos = Self::get_min_chunk_pos(pos);

        let mut chunks = std::array::from_fn(|_| std::array::from_fn(|_| Suspendable::Placeholder));
        for x in 0..CHUNKS_X {
            for y in 0..CHUNKS_Y {
                let local_chunk_pos = vec2!(x as i32, y as i32);

                let chunk_pos = min_chunk_pos + local_chunk_pos;

                let offset_from_center = local_chunk_pos - Self::CENTER_CHUNK_OFFSET;

                let chunk = T::load(chunk_pos, offset_from_center, &mut ctx);

                chunks[y][x] = Suspendable::Some(chunk);
            }
        }

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
                    let offset_from_center = vec2!(x as i32, y as i32) - Self::CENTER_CHUNK_OFFSET;

                    let chunk_pos_in_old =
                        vec2!(x as i32, y as i32) + min_chunk_pos - old_min_chunk_pos;

                    let reuse_x =
                        chunk_pos_in_old.x() >= 0 && chunk_pos_in_old.x() < CHUNKS_X as i32;
                    let reuse_y =
                        chunk_pos_in_old.y() >= 0 && chunk_pos_in_old.y() < CHUNKS_Y as i32;

                    self.chunks[y][x] = if reuse_x && reuse_y {
                        let x_in_old = chunk_pos_in_old.x() as usize;
                        let y_in_old = chunk_pos_in_old.y() as usize;

                        match &mut old_chunks[y_in_old][x_in_old] {
                            Suspendable::Some(chunk) => chunk.moved(offset_from_center, &mut ctx),
                            Suspendable::Placeholder => {}
                        };

                        replace(
                            &mut old_chunks[y_in_old][x_in_old],
                            Suspendable::Placeholder,
                        )
                    } else {
                        let local_chunk_pos = vec2!(x as i32, y as i32);

                        let chunk_pos = min_chunk_pos + local_chunk_pos;

                        let offset_from_center = local_chunk_pos - Self::CENTER_CHUNK_OFFSET;

                        Suspendable::Some(T::load(chunk_pos, offset_from_center, &mut ctx))
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
