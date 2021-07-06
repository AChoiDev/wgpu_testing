pub struct ResourceViews<'a> {
    pub color: wgpu::TextureView,
    pub depth: wgpu::TextureView,
    pub trace_frame: wgpu::BufferSlice<'a>,

    pub map: wgpu::TextureView,
    pub oct_map: wgpu::TextureView,
    pub mono_bit_map: wgpu::TextureView,
    pub default_sampler: &'a wgpu::Sampler,
    pub mono_bit_map_mipmaps: Vec<wgpu::TextureView>,
}

impl<'a> ResourceViews<'a> {
    pub fn new(resources: &'a super::resources::Resources)
    -> Self {
        Self {
            mono_bit_map_mipmaps:
                (0..4)
                .into_iter()
                .map(|i|
                    resources.mono_bit_map_texture
                    .create_view(&wgpu::TextureViewDescriptor {
                        base_mip_level: i,
                        level_count: std::num::NonZeroU32::new(1),
                        ..Default::default()
                    })
                ).collect(),
            color:
                resources.render_textures.color
                .create_view(&wgpu::TextureViewDescriptor::default()),
            depth:
                resources.render_textures.depth
                .create_view(&wgpu::TextureViewDescriptor::default()),
            oct_map:
                resources.oct_map_texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
            map:
                resources.map_texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
            trace_frame:
                resources.buffers.trace_frame
                .slice(..),
            mono_bit_map:
                resources.mono_bit_map_texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
            default_sampler:
                &resources.default_sampler,
        }
    }
}