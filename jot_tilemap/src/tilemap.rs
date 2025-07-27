use std::collections::HashMap;

use super::*;

#[derive(Debug, Clone)]
pub struct Tilemap<const CHUNK_HEIGHT: u32, V: Visual2D, T: TileTransform2D> {
    chunks: HashMap<i32, TilemapChunk<CHUNK_HEIGHT, V, T>>,
}

pub struct StructuredTiles<const CHUNK_HEIGHT: u32, V: Visual2D, T: TileTransform2D> {
    /// Please make sure that the tiles are sorted bottom to top, left to right, column by column.
    ///
    /// Please also make sure tile chunks are calculated as `tile_y.floor_div(CHUNK_HEIGHT)`.
    pub chunks: HashMap<i32, Vec<Quad2D<V, T>>>,
}

#[derive(Debug, Clone)]
struct TilemapChunk<const CHUNK_HEIGHT: u32, V: Visual2D, T: TileTransform2D> {
    tiles_buf: GpuBuffer<[Quad2D<V, T>]>,
    first_column: i32,
    column_start_indicies: Vec<usize>,
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
        let chunks = tiles
            .chunks
            .iter()
            .map(|(&k, chunk)| {
                let tiles_buf = gpu.create_buffer(GpuBufferDesc {
                    label: Some("Tilemap Chunk Buffer"),
                    usages: GpuBufferUsages::VERTEX,
                    value: chunk.as_slice(),
                });

                let mut first_column = 0;
                let mut column_start_indicies = Vec::new();

                for (tile_idx, tile) in chunk.iter().enumerate() {
                    let tile_pos = tile.transform.tile_pos();
                    let tile_column = tile_pos.x();

                    if tile_idx == 0 {
                        first_column = tile_column;
                    }

                    // purposefully filling in the gaps between columns.
                    while {
                        let last_column = first_column + column_start_indicies.len() as i32 - 1;
                        let is_new_column = tile_column != last_column;

                        is_new_column
                    } {
                        column_start_indicies.push(tile_idx);
                    }
                }

                let v = TilemapChunk {
                    tiles_buf,
                    first_column,
                    column_start_indicies,
                };

                (k, v)
            })
            .collect::<HashMap<_, _>>();

        Self { chunks }
    }

    pub fn render<C: TilemapCamera2D>(
        &self,
        input: &TilemapRenderInput<CHUNK_HEIGHT, V, T, C>,
        output: &GpuTexture<2>,
        renderer: &Renderer2D<V, T, C>,
        gpu: &Gpu,
    ) {
        let visible_columns = input.cam.visible_tile_columns();

        for chunk_row in input.cam.visible_tile_chunks::<CHUNK_HEIGHT>() {
            let chunk = match self.chunks.get(&chunk_row) {
                Some(chunk) => chunk,
                None => continue,
            };

            let start_column_idx = (visible_columns.start - chunk.first_column)
                .clamp(0, chunk.column_start_indicies.len() as i32)
                as usize;

            let end_column_idx = (visible_columns.end - chunk.first_column)
                .clamp(0, chunk.column_start_indicies.len() as i32)
                as usize;

            let tile_indicies = chunk.column_start_indicies[start_column_idx]
                ..chunk.column_start_indicies[end_column_idx];

            renderer.render(
                RenderInput2D {
                    cam_bind_group: input.cam_bind_group,
                    background_color: input.background_color,
                    quads: chunk.tiles_buf.slice(tile_indicies),
                    visual_bind_group: input.visual_bind_group,
                    transform_bind_group: input.transform_bind_group,
                },
                output,
                gpu,
            );
        }
    }
}

impl<const CHUNK_HEIGHT: u32, V: Visual2D, T: TileTransform2D> StructuredTiles<CHUNK_HEIGHT, V, T> {
    pub fn new(tiles: &[Quad2D<V, T>]) -> Self {
        let mut chunks = HashMap::<i32, Vec<Quad2D<V, T>>>::new();

        for &tile in tiles {
            let tile_chunk = tile.transform.tile_chunk::<CHUNK_HEIGHT>();

            chunks.entry(tile_chunk).or_default().push(tile);
        }

        for chunk in chunks.values_mut() {
            chunk.sort_by(|a, b| a.transform.tile_sort(&b.transform));
        }

        Self { chunks }
    }
}
