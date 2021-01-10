
pub struct BindGroups {
    pub map: wgpu::BindGroup,
    pub edit_sum_table: wgpu::BindGroup,
    pub view: wgpu::BindGroup,
    pub edit_mono_bit_map_texture: wgpu::BindGroup,
    pub march: wgpu::BindGroup,
    pub cone_march: wgpu::BindGroup,
    pub depth_shade: wgpu::BindGroup,
    pub process: wgpu::BindGroup,
    pub halve_map_binds: Vec<wgpu::BindGroup>,
}

impl BindGroups {
    pub fn new(
        device: &wgpu::Device, 
        bind_group_layouts: &super::bind_group_layouts::BindGroupLayouts,
        views: &super::resource_views::ResourceViews,
    ) 
    -> Self {
        Self {
            march:
                device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &bind_group_layouts.march,
                        entries: &march_entries(&views)
                    }
                ),
            cone_march:
                device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &bind_group_layouts.cone_march,
                        entries: &cone_march_entries(&views)
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
            view: 
                device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &bind_group_layouts.view,
                        entries: &view_entries(&views),
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
            map:
                device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &bind_group_layouts.map,
                        entries: &map_entries(&views)
                    }
                ),
            edit_mono_bit_map_texture:
                device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &bind_group_layouts.edit_map,
                        entries: &edit_mono_bit_map_entries(&views)
                    }
                ),
            depth_shade:
                device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &bind_group_layouts.depth_shade,
                        entries: &depth_shade_entries(&views)
                    }
                ),
            halve_map_binds:
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
    }
}

fn edit_mono_bit_map_entries<'a>(views: &'a super::resource_views::ResourceViews) 
-> Vec<wgpu::BindGroupEntry<'a>> {
    make_entries(
        vec![
            wgpu::BindingResource::TextureView(
                &views.mono_bit_map,
            )
        ]
    )
}

fn depth_shade_entries<'a>(views: &'a super::resource_views::ResourceViews) 
-> Vec<wgpu::BindGroupEntry<'a>> {
    make_entries(
        vec![
            wgpu::BindingResource::TextureView(
                &views.depth,
            ),
            wgpu::BindingResource::TextureView(
                &views.color,
            ),
        ]
    )
}

fn view_entries<'a>(views: &'a super::resource_views::ResourceViews) 
-> Vec<wgpu::BindGroupEntry<'a>> {
    make_entries(
        vec![
            wgpu::BindingResource::Buffer(
                views.trace_frame,
            ),
        ]
    )
}
fn map_entries<'a>(views: &'a super::resource_views::ResourceViews) 
-> Vec<wgpu::BindGroupEntry<'a>> {
    make_entries(
        vec![
            wgpu::BindingResource::TextureView(
                &views.map,
            ),
        ]
    )
}

pub fn make_edit_sum_table_entries<'a>(views: &'a super::resource_views::ResourceViews) 
-> Vec<wgpu::BindGroupEntry<'a>> {
    make_entries(
        vec![
            wgpu::BindingResource::TextureView(
                &views.sum_map,
            ),
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

pub fn cone_march_entries<'a>(views: &'a super::resource_views::ResourceViews) 
-> Vec<wgpu::BindGroupEntry<'a>> {
    make_entries(
        vec![
            wgpu::BindingResource::TextureView(
                &views.cone_depth,
            ),
            wgpu::BindingResource::TextureView(
                &views.mono_bit_map,
            ),
            wgpu::BindingResource::Sampler(
                &views.mono_bit_map_sampler,
            ),
        ]
    )
}

pub fn march_entries<'a>(views: &'a super::resource_views::ResourceViews) 
-> Vec<wgpu::BindGroupEntry<'a>> {
    make_entries(
        vec![
            wgpu::BindingResource::TextureView(
                &views.sum_map,
            ),
            wgpu::BindingResource::TextureView(
                &views.depth,
            ),
            wgpu::BindingResource::TextureView(
                &views.mono_bit_map,
            ),
            wgpu::BindingResource::Sampler(
                &views.mono_bit_map_sampler,
            ),
            wgpu::BindingResource::TextureView(
                &views.cone_depth,
            ),
        ]
    )
}

pub fn make_process_entries<'a>(views: &'a super::resource_views::ResourceViews) 
-> Vec<wgpu::BindGroupEntry<'a>> {
    make_entries(
        vec![
            wgpu::BindingResource::TextureView(&views.color)
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
