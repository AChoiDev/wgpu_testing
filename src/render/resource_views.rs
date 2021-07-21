pub struct ResourceViews<'a> {
    pub color: wgpu::TextureView,
    pub depth: wgpu::TextureView,
    pub trace_frame: wgpu::BufferSlice<'a>,
    pub chunk_changes: wgpu::BufferSlice<'a>,

    pub map: wgpu::TextureView,
    pub mono_bit_map: wgpu::TextureView,
    pub default_sampler: &'a wgpu::Sampler,
    pub mono_bit_map_mipmaps: Vec<wgpu::TextureView>,
    pub layer_index_map: wgpu::TextureView,
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
            map:
                resources.map_texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
            trace_frame:
                resources.buffers.trace_frame
                .slice(..),
            chunk_changes:
                resources.buffers.chunk_changes
                .slice(..),
            mono_bit_map:
                resources.mono_bit_map_texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
            default_sampler:
                &resources.default_sampler,
            layer_index_map:
                resources.layer_index_map_texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
        }
    }
}