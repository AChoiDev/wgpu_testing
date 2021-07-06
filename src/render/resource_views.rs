pub struct ResourceViews<'a> {
    pub color: wgpu::TextureView,
    pub depth: wgpu::TextureView,
    pub trace_frame: wgpu::BufferSlice<'a>,

    pub oct_map: wgpu::TextureView,
    pub default_sampler: &'a wgpu::Sampler,
}

impl<'a> ResourceViews<'a> {
    pub fn new(resources: &'a super::resources::Resources)
    -> Self {
        Self {
            color:
                resources.render_textures.color
                .create_view(&wgpu::TextureViewDescriptor::default()),
            depth:
                resources.render_textures.depth
                .create_view(&wgpu::TextureViewDescriptor::default()),
            oct_map:
                resources.oct_map_texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
            trace_frame:
                resources.buffers.trace_frame
                .slice(..),
            default_sampler:
                &resources.default_sampler,
        }
    }
}