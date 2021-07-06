pub struct ResourceViews<'a> {
    pub color: wgpu::TextureView,
    pub depth: wgpu::TextureView,
    pub trace_frame: wgpu::BufferSlice<'a>,
    pub upload_map_counter: wgpu::BufferSlice<'a>,

    pub oct_map: wgpu::TextureView,
    pub index_map: wgpu::TextureView,
    pub default_sampler: &'a wgpu::Sampler,
    pub upload_map: wgpu::TextureView,
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
            index_map:
                resources.displacement_index_map
                .create_view(&wgpu::TextureViewDescriptor::default()),
            default_sampler:
                &resources.default_sampler,
            upload_map:
                resources.upload_map_texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
            upload_map_counter:
                resources.buffers.upload_map_counter
                .slice(..),
        }
    }
}