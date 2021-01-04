pub struct BindGroups {
    pub trace: wgpu::BindGroup,
    pub process: wgpu::BindGroup,
    pub edit_sum_table: wgpu::BindGroup,
}

impl BindGroups {
    pub fn new(
        device: &wgpu::Device, 
        bind_group_layouts: &super::bind_group_layouts::BindGroupLayouts,
        views: &super::resources::ResourceViews,
    ) 
    -> Self {
        Self {
            trace:
                device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &bind_group_layouts.trace,
                        entries: &make_trace_entries(&views)
                    }
                ),
            process:
                device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &bind_group_layouts.process,
                        entries: &make_process_entries(&views),
                    }
                ),
            edit_sum_table:
                device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &bind_group_layouts.edit_sum_table,
                        entries: &make_edit_sum_table_entries(&views)
                    }
                ),
        }
    }
}
pub fn make_edit_sum_table_entries<'a>(views: &'a super::resources::ResourceViews) 
-> Vec<wgpu::BindGroupEntry<'a>> {
    make_entries(
        vec![
            wgpu::BindingResource::TextureView(
                &views.map
            ),
            wgpu::BindingResource::TextureView(
                &views.sum_map,
            ),
        ]
    )
}


pub fn make_trace_entries<'a>(views: &'a super::resources::ResourceViews) 
-> Vec<wgpu::BindGroupEntry<'a>> {
    make_entries(
        vec![
            wgpu::BindingResource::TextureView(
                &views.color
            ),
            wgpu::BindingResource::TextureView(
                &views.map
            ),
            wgpu::BindingResource::TextureView(
                &views.sum_map,
            ),
            wgpu::BindingResource::Buffer(
                views.trace_frame
            ),
        ]
    )
}

pub fn make_process_entries<'a>(views: &'a super::resources::ResourceViews) 
-> Vec<wgpu::BindGroupEntry<'a>> {
    make_entries(
        vec![
            wgpu::BindingResource::TextureView(
                &views.color
            )
        ]
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