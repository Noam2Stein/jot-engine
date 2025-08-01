use std::{cmp::Ordering, collections::HashMap, ops::Range};

use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone)]
pub struct Tilemap<const CHUNK_HEIGHT: u32, V: Visual2D, T: TileTransform2D> {
    tile_buf: GpuBuffer<[Quad2D<V, T>]>,
    chunks: HashMap<i32, TilemapChunk<CHUNK_HEIGHT>>,
}

/// Please make sure that the tiles are sorted bottom to top, left to right, column by column.
///
/// Please also make sure tile chunks are calculated as `tile_y.floor_div(CHUNK_HEIGHT)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredTiles<const CHUNK_HEIGHT: u32, V: Visual2D, T: TileTransform2D> {
    pub tiles: Vec<Quad2D<V, T>>,
    /// For each chunk, the range indicies from `tiles` that are in the chunk.
    pub chunks: HashMap<i32, Range<usize>>,
}

#[derive(Debug, Clone)]
struct TilemapChunk<const CHUNK_HEIGHT: u32> {
    first_column: i32,
    columns: Vec<TilemapColumnInfo>,
}

#[derive(Debug, Clone)]
struct TilemapColumnInfo {
    start_idx: usize,
}

impl<const CHUNK_HEIGHT: u32, V: Visual2D, T: TileTransform2D> Tilemap<CHUNK_HEIGHT, V, T> {
    pub fn new(tiles: &[Quad2D<V, T>], gpu: &Gpu) -> Self {
        let structured_tiles = StructuredTiles::new(tiles);

        Self::new_structured(&structured_tiles, gpu)
    }

    /// Please make sure that the tiles are sorted bottom to top, left to right, column by column.
    ///
    /// Please also make sure tile chunks are calculated as `tile_y.floor_div(CHUNK_HEIGHT)`.
    pub fn new_structured(tiles: &StructuredTiles<CHUNK_HEIGHT, V, T>, gpu: &Gpu) -> Self {
        #[cfg(debug_assertions)]
        {
            for (tile_a, tile_b) in tiles.tiles.iter().zip(tiles.tiles.iter().skip(1)) {
                if tile_cmp::<CHUNK_HEIGHT, V, T>(tile_a, tile_b) != Ordering::Less {
                    panic!(
                        "StructuredTiles is not sorted properly. {:?} {:?}",
                        tile_a, tile_b
                    );
                }
            }
        }

        let tile_buf = gpu.create_buffer(GpuBufferDesc {
            label: Some("Tilemap Chunk Buffer"),
            usages: GpuBufferUsages::VERTEX,
            value: tiles.tiles.as_slice(),
        });

        let chunks = tiles
            .chunks
            .iter()
            .map(|(&k, chunk_range)| {
                let chunk_tiles = &tiles.tiles[chunk_range.clone()];

                #[cfg(debug_assertions)]
                {
                    for (tile_a, tile_b) in chunk_tiles.iter().zip(chunk_tiles.iter().skip(1)) {
                        if tile_cmp_in_chunk::<CHUNK_HEIGHT, V, T>(tile_a, tile_b) != Ordering::Less
                        {
                            panic!(
                                "StructuredTiles chunk is not sorted properly. {:?} {:?}",
                                tile_a, tile_b
                            );
                        }
                    }
                }

                let mut first_column = 0;
                let mut columns = Vec::new();

                for (tile_idx_in_chunk, tile) in chunk_tiles.iter().enumerate() {
                    let tile_idx = chunk_range.start + tile_idx_in_chunk;

                    let tile_pos = tile.transform.tile_pos();
                    let tile_column = tile_pos.x();

                    if tile_idx_in_chunk == 0 {
                        first_column = tile_column;
                    }

                    // purposefully filling in the gaps between columns.
                    while {
                        let last_column = first_column + columns.len() as i32 - 1;
                        let is_new_column = tile_column != last_column;

                        is_new_column
                    } {
                        columns.push(TilemapColumnInfo {
                            start_idx: tile_idx,
                        });
                    }
                }

                columns.push(TilemapColumnInfo {
                    start_idx: chunk_range.end,
                });

                let v = TilemapChunk {
                    first_column,
                    columns,
                };

                (k, v)
            })
            .collect::<HashMap<_, _>>();

        Self { tile_buf, chunks }
    }

    pub fn render<C: TilemapCamera2D>(
        &self,
        input: &TilemapRenderInput<CHUNK_HEIGHT, V, T, C>,
        output: &GpuTexture<2>,
        renderer: &Renderer2D<V, T, C>,
        gpu: &Gpu,
    ) {
        let aspect = output.size().x() as f32 / output.size().y() as f32;

        let mut background_color = input.background_color;

        let visible_chunks = input.cam.visible_tile_chunks::<CHUNK_HEIGHT>(aspect);
        let visible_columns = input.cam.visible_tile_columns(aspect);

        for chunk_row in visible_chunks {
            let chunk = match self.chunks.get(&chunk_row) {
                Some(chunk) => chunk,
                None => continue,
            };

            let start_column_idx = (visible_columns.start - chunk.first_column)
                .clamp(0, chunk.columns.len() as i32 - 1)
                as usize;

            let end_column_idx = (visible_columns.end - chunk.first_column)
                .clamp(0, chunk.columns.len() as i32 - 1) as usize;

            let start_column = &chunk.columns[start_column_idx];
            let end_column = &chunk.columns[end_column_idx];

            let tile_indicies = start_column.start_idx..end_column.start_idx;

            renderer.render(
                RenderInput2D {
                    cam_bind_group: input.cam_bind_group,
                    background_color,
                    quads: self.tile_buf.slice(tile_indicies),
                    visual_bind_group: input.visual_bind_group,
                    transform_bind_group: input.transform_bind_group,
                },
                output,
                gpu,
            );

            background_color = None;
        }
    }
}

impl<const CHUNK_HEIGHT: u32, V: Visual2D, T: TileTransform2D> StructuredTiles<CHUNK_HEIGHT, V, T> {
    pub fn new(unstructured_tiles: &[Quad2D<V, T>]) -> Self {
        let mut tiles = Vec::from_iter(unstructured_tiles.iter().copied());

        tiles.sort_by(tile_cmp::<CHUNK_HEIGHT, V, T>);

        let mut chunks = HashMap::<i32, Range<usize>>::new();

        for (tile_idx, tile) in tiles.iter().enumerate() {
            let tile_chunk = tile_chunk::<CHUNK_HEIGHT>(tile.transform.tile_pos());

            chunks.entry(tile_chunk).or_insert(tile_idx..tile_idx).end = tile_idx + 1;
        }

        Self { tiles, chunks }
    }
}

fn tile_chunk<const CHUNK_HEIGHT: u32>(tile_pos: IVec2) -> i32 {
    (s32::from_i32(tile_pos.y()) / s32::from_u32(CHUNK_HEIGHT))
        .floor()
        .as_i32()
}

fn tile_cmp<const CHUNK_HEIGHT: u32, V: Visual2D, T: TileTransform2D>(
    a: &Quad2D<V, T>,
    b: &Quad2D<V, T>,
) -> Ordering {
    let a_pos = a.transform.tile_pos();
    let b_pos = b.transform.tile_pos();
    let a_chunk = tile_chunk::<CHUNK_HEIGHT>(a_pos);
    let b_chunk = tile_chunk::<CHUNK_HEIGHT>(b_pos);

    if a_chunk != b_chunk {
        a_chunk.cmp(&b_chunk)
    } else {
        tile_cmp_in_chunk::<CHUNK_HEIGHT, V, T>(a, b)
    }
}

fn tile_cmp_in_chunk<const CHUNK_HEIGHT: u32, V: Visual2D, T: TileTransform2D>(
    a: &Quad2D<V, T>,
    b: &Quad2D<V, T>,
) -> Ordering {
    let a_pos = a.transform.tile_pos();
    let b_pos = b.transform.tile_pos();

    if a_pos.x() != b_pos.x() {
        a_pos.x().cmp(&b_pos.x())
    } else {
        a_pos.y().cmp(&b_pos.y())
    }
}
