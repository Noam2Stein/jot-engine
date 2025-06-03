use super::*;

pub fn clear(color: FVec4, output: &GPUTextureView, ctx: &GPUContext) {
    let mut encoder = ctx.device.create_command_encoder(&GPUCommandEncoderDesc {
        label: Some("Clear Encoder"),
    });

    encode_clear(color, output, &mut encoder);

    ctx.queue.submit(std::iter::once(encoder.finish()));
}

pub fn encode_clear(color: FVec4, output: &GPUTextureView, encoder: &mut GPUCommandEncoder) {
    {
        encoder.begin_render_pass(&GPURenderPassDesc {
            label: Some("Renderer - Render Pass"),
            color_attachments: &[Some(GPURenderPassColorAttachment {
                view: output,
                resolve_target: None,
                ops: GPUOps {
                    load: GPULoadOp::Clear(GPUColor {
                        r: color.x.into(),
                        g: color.y.into(),
                        b: color.z.into(),
                        a: 0.0,
                    }),
                    store: GPUStoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
    }
}
