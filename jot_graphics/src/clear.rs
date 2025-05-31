use super::*;

pub fn clear(color: Vec4, output: &TextureView, ctx: &GPUContext) {
    let mut encoder = ctx
        .device
        .create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Clear Encoder"),
        });

    encode_clear(color, output, &mut encoder);

    ctx.queue.submit(std::iter::once(encoder.finish()));
}

pub fn encode_clear(color: Vec4, output: &TextureView, encoder: &mut CommandEncoder) {
    {
        encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Renderer - Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: output,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(GPUColor {
                        r: color.x.into(),
                        g: color.y.into(),
                        b: color.z.into(),
                        a: 0.0,
                    }),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
    }
}
