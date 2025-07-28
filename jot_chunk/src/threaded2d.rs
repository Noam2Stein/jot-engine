use std::{collections::HashMap, mem::replace, thread::JoinHandle};

use super::*;

/// A trait for chunks that are loaded in a separate thread.
///
/// `prepare` is the only fn that is called on a seperate thread, and must not have sideeffects.
/// If a chunk that is not prepared yet is not needed anymore, `load` and `unload` will NOT be called.
pub trait ThreadedChunk2D: Send + Sync + 'static {
    type Context<'a>;
    type PrepareContext: Send + Sync + Clone + 'static;
    type Prepare: Send + Sync + 'static;

    fn prepare(chunk_pos: IVec2, ctx: Self::PrepareContext) -> Self::Prepare;

    fn load(chunk_pos: IVec2, prepared: Self::Prepare, ctx: &mut Self::Context<'_>) -> Self;

    fn unload(self, ctx: &mut Self::Context<'_>);
}

#[derive(Debug)]
pub struct ThreadedChunkHolder2D<
    T: ThreadedChunk2D,
    const CHUNK_WIDTH: u32,
    const CHUNK_HEIGHT: u32,
    const CHUNKS_X: usize,
    const CHUNKS_Y: usize,
    const NEEDED_X: usize,
    const NEEDED_Y: usize,
> {
    inner: ChunkHolder2D<
        ChunkWrapper<T, NEEDED_X, NEEDED_Y>,
        CHUNK_WIDTH,
        CHUNK_HEIGHT,
        CHUNKS_X,
        CHUNKS_Y,
    >,
    deleting_chunks: HashMap<IVec2, JoinHandle<T::Prepare>>,
}

impl<
    T: ThreadedChunk2D,
    const CHUNK_WIDTH: u32,
    const CHUNK_HEIGHT: u32,
    const CHUNKS_X: usize,
    const CHUNKS_Y: usize,
    const NEEDED_X: usize,
    const NEEDED_Y: usize,
> ThreadedChunkHolder2D<T, CHUNK_WIDTH, CHUNK_HEIGHT, CHUNKS_X, CHUNKS_Y, NEEDED_X, NEEDED_Y>
{
    pub fn new(pos: SVec2, ctx: T::Context<'_>, prepare_ctx: &T::PrepareContext) -> Self {
        let mut deleting_chunks = HashMap::new();

        let inner = ChunkHolder2D::new(
            pos,
            ContextWrapper {
                deleting_chunks: transmute_lifetime_mut(&mut deleting_chunks),
                ctx,
                prepare_ctx: transmute_lifetime(prepare_ctx),
            },
        );

        Self {
            inner,
            deleting_chunks,
        }
    }

    pub fn update(&mut self, pos: SVec2, ctx: T::Context<'_>, prepare_ctx: &T::PrepareContext) {
        let ctx_wrapper = ContextWrapper {
            deleting_chunks: transmute_lifetime_mut(&mut self.deleting_chunks),
            ctx,
            prepare_ctx: transmute_lifetime(prepare_ctx),
        };

        self.inner.target_moved(pos, ctx_wrapper);

        self.deleting_chunks
            .retain(|_, handle| !handle.is_finished());
    }
}

#[derive(Debug)]
enum ChunkWrapper<T: ThreadedChunk2D, const NEEDED_X: usize, const NEEDED_Y: usize> {
    Preparing {
        join_handle: JoinHandle<T::Prepare>,
        chunk_pos: IVec2,
    },
    Loaded(T),
    Temporary,
}

struct ContextWrapper<'a, T: ThreadedChunk2D> {
    deleting_chunks: &'a mut HashMap<IVec2, JoinHandle<T::Prepare>>,
    ctx: T::Context<'a>,
    prepare_ctx: &'a T::PrepareContext,
}

impl<T: ThreadedChunk2D, const NEEDED_X: usize, const NEEDED_Y: usize> Chunk2D
    for ChunkWrapper<T, NEEDED_X, NEEDED_Y>
{
    type Context<'a> = ContextWrapper<'a, T>;

    fn load(chunk_pos: IVec2, offset_from_center: IVec2, ctx: &mut Self::Context<'_>) -> Self {
        let needed_offset_x = (NEEDED_X as i32 - 1) / 2;
        let needed_offset_y = (NEEDED_Y as i32 - 1) / 2;

        let join_handle = 'get_join_handle: {
            if let Some(handle) = ctx.deleting_chunks.remove(&chunk_pos) {
                break 'get_join_handle handle;
            }

            let ctx_clone = ctx.prepare_ctx.clone();
            let join_handle = std::thread::spawn(move || T::prepare(chunk_pos, ctx_clone));

            join_handle
        };

        let is_needed = offset_from_center.x().abs() <= needed_offset_x
            && offset_from_center.y().abs() <= needed_offset_y;

        if is_needed {
            let prepared = join_handle.join().unwrap();

            ChunkWrapper::Loaded(T::load(chunk_pos, prepared, &mut ctx.ctx))
        } else {
            Self::Preparing {
                join_handle,
                chunk_pos,
            }
        }
    }

    fn moved(&mut self, offset_from_center: IVec2, ctx: &mut Self::Context<'_>) {
        match self {
            ChunkWrapper::Preparing {
                join_handle: _,
                chunk_pos,
            } => {
                let chunk_pos = *chunk_pos;

                let needed_offset_x = (NEEDED_X as i32 - 1) / 2;
                let needed_offset_y = (NEEDED_Y as i32 - 1) / 2;

                let is_needed = offset_from_center.x().abs() <= needed_offset_x
                    && offset_from_center.y().abs() <= needed_offset_y;

                if is_needed {
                    let join_handle = match replace(self, Self::Temporary) {
                        ChunkWrapper::Preparing { join_handle, .. } => join_handle,

                        _ => unreachable!(),
                    };

                    let prepared = join_handle.join().unwrap();
                    *self = Self::Loaded(T::load(chunk_pos, prepared, &mut ctx.ctx));

                    return;
                }
            }

            ChunkWrapper::Loaded(_) => {}

            ChunkWrapper::Temporary => unreachable!(),
        }
    }

    fn unload(self, ctx: &mut Self::Context<'_>) {
        match self {
            ChunkWrapper::Loaded(chunk) => {
                chunk.unload(&mut ctx.ctx);
            }

            ChunkWrapper::Preparing {
                join_handle,
                chunk_pos,
            } => {
                ctx.deleting_chunks.insert(chunk_pos, join_handle);
            }

            ChunkWrapper::Temporary => unreachable!(),
        }
    }
}

fn transmute_lifetime<'a, 'b, T: 'a>(x: &'a T) -> &'b T {
    unsafe { std::mem::transmute(x) }
}

fn transmute_lifetime_mut<'a, 'b, T: 'a>(x: &'a mut T) -> &'b mut T {
    unsafe { std::mem::transmute(x) }
}
