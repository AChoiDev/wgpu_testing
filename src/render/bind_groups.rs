
pub struct BindGroups {
    pub primary: wgpu::BindGroup,
    pub halve_map_binds: Vec<wgpu::BindGroup>,
    pub edit_mono_bit_map_texture: wgpu::BindGroup,
}

impl BindGroups {
    pub fn new(
        device: &wgpu::Device, 
        bind_group_layouts: &super::bind_group_layouts::BindGroupLayouts,
        views: &super::resource_views::ResourceViews,
    ) 
    -> Self {
        Self {
            edit_mono_bit_map_texture:
                device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &bind_group_layouts.edit_map,
                        entries: &edit_mono_bit_map_entries(&views)
                    }
                ),
            primary: create_primary_binds(&device, &views, &bind_group_layouts),
            halve_map_binds: halve_map_binds(&device, &views, &bind_group_layouts),
        }
    }
}
fn halve_map_binds<'a>(device: &wgpu::Device, views: &'a super::resource_views::ResourceViews, bind_group_layouts: &super::bind_group_layouts::BindGroupLayouts) -> Vec<wgpu::BindGroup> {
    (1..(super::resources::MONO_BIT_LEVELS))
    .into_iter()
    .map(|i|
        device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: None,
                layout: &bind_group_layouts.halve_map,
                entries: &halve_map_entries(&views, i as usize),
            }
        )
    ).collect()
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
                    wgpu::BindingResource::Sampler(
                        &views.default_sampler,
                    ),
                    wgpu::BindingResource::TextureView(
                        &views.mono_bit_map,
                    ),
                    wgpu::BindingResource::TextureView(
                        &views.map,
                    ),
                    wgpu::BindingResource::Buffer(
                        views.trace_frame
                    ),
                    wgpu::BindingResource::TextureView(
                        &views.layer_index_map
                    ),
                ]
            )
        }
    )
}


fn edit_mono_bit_map_entries<'a>(views: &'a super::resource_views::ResourceViews) 
-> Vec<wgpu::BindGroupEntry<'a>> {
    make_entries(
        vec![
            wgpu::BindingResource::TextureView(
                &views.mono_bit_map,
            ),
            wgpu::BindingResource::Buffer(views.chunk_changes),
        ]
    )
}

pub fn halve_map_entries<'a>(views: &'a super::resource_views::ResourceViews, target_level: usize) 
-> Vec<wgpu::BindGroupEntry<'a>> {
    make_entries(
        vec![
            wgpu::BindingResource::TextureView(
                &views.mono_bit_map_mipmaps[target_level - 1],
            ),
            wgpu::BindingResource::TextureView(
                &views.mono_bit_map_mipmaps[target_level],
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
