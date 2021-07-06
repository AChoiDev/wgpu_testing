
pub struct BindGroups {
    pub primary: wgpu::BindGroup,
    pub uploadMap: wgpu::BindGroup,
}

impl BindGroups {
    pub fn new(
        device: &wgpu::Device, 
        bind_group_layouts: &super::bind_group_layouts::BindGroupLayouts,
        views: &super::resource_views::ResourceViews,
    ) 
    -> Self {
        Self {
            primary: create_primary_binds(&device, &views, &bind_group_layouts),
            uploadMap: create_upload_map_binds(&device, &views, &bind_group_layouts)
        }
    }
}

fn create_upload_map_binds<'a>(device: &wgpu::Device, views: &'a super::resource_views::ResourceViews, bind_group_layouts: &super::bind_group_layouts::BindGroupLayouts) -> wgpu::BindGroup {
    device.create_bind_group(
        &wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layouts.upload_map,
            entries: &make_entries(
                vec![
                    wgpu::BindingResource::Buffer(
                        views.upload_map_counter
                    ),
                    wgpu::BindingResource::TextureView(
                        &views.upload_map,
                    ),
                ]
            )
        }
    )
}



fn create_primary_binds<'a>(device: &wgpu::Device, views: &'a super::resource_views::ResourceViews, bind_group_layouts: &super::bind_group_layouts::BindGroupLayouts) -> wgpu::BindGroup {
    device.create_bind_group(
        &wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layouts.primary_layout,
            entries: &make_entries(
                vec![
                    wgpu::BindingResource::TextureView(
                        &views.depth,
                    ),
                    wgpu::BindingResource::TextureView(
                        &views.color,
                    ),
                    wgpu::BindingResource::Buffer(
                        views.trace_frame
                    ),
                    wgpu::BindingResource::TextureView(
                        &views.oct_map,
                    ),
                    wgpu::BindingResource::TextureView(
                        &views.index_map,
                    ),
                ]
            )
        }
    )
}



pub fn make_entries<'a>(resources: Vec<wgpu::BindingResource<'a>>)
-> Vec<wgpu::BindGroupEntry<'a>> {
    resources
    .into_iter()
    .enumerate()
    .map(|(i, br)|
        wgpu::BindGroupEntry {
            binding: i as u32,
            resource: br
        }
    ).collect()
}
