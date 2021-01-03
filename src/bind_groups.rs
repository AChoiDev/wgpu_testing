pub struct BindGroups {
    pub trace: wgpu::BindGroup,
    pub process: wgpu::BindGroup,
}

impl BindGroups {
    pub fn new(
        device: &wgpu::Device, 
        bind_group_layouts: &crate::bind_group_layouts::BindGroupLayouts,
        resources: &crate::resources::Resources,
    ) 
    -> Self {
        Self {
            trace:
                device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &bind_group_layouts.trace,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: 
                                    wgpu::BindingResource::TextureView(
                                        // MAKE A TEXTURE VIEW or BINDING RESOURCE STRUCT
                                        &resources.render_textures.color()
                                    )
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: 
                                    wgpu::BindingResource::TextureView(
                                        &resources.map_texture()
                                    )
                            },
                            wgpu::BindGroupEntry {
                                binding: 2,
                                resource: 
                                    wgpu::BindingResource::Buffer(
                                        resources.buffers.trace_frame.slice(..),
                                    )
                            },
                        ],
                    }
                ),
            process:
                device.create_bind_group(
                &wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &bind_group_layouts.process,
                    entries: 
                        &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource:
                                    wgpu::BindingResource::TextureView(
                                        &resources.render_textures.color()
                                    )
                            }
                        ],
                    }
                ),
        }
    }
}






