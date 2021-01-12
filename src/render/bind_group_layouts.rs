

pub struct BindGroupLayouts {
    pub map: wgpu::BindGroupLayout,
    pub edit_map: wgpu::BindGroupLayout,
    pub edit_sum_table: wgpu::BindGroupLayout,
    pub view: wgpu::BindGroupLayout,
    pub march: wgpu::BindGroupLayout,
    pub cone_march: wgpu::BindGroupLayout,
    pub depth_shade: wgpu::BindGroupLayout,
    pub process: wgpu::BindGroupLayout,
    pub halve_map: wgpu::BindGroupLayout,
}

impl BindGroupLayouts {
    pub fn new(device: &wgpu::Device)
    -> Self {
        Self {
            map: map_layout(&device),
            edit_sum_table: edit_sum_table_layout(&device),
            view: view_layout(&device),
            march: march_layout(&device),
            cone_march: cone_march_layout(&device),
            depth_shade: depth_shade_layout(&device),
            process: process_layout(&device),
            edit_map: edit_map_layout(&device),
            halve_map: halve_map_layout(&device),
        }
    }
}

fn cone_march_layout(device: &wgpu::Device) 
-> wgpu::BindGroupLayout {
    bind_group_layout_compute(&device, 
        &[
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D2,
                format: wgpu::TextureFormat::R32Float,
                readonly: false,
            },
            wgpu::BindingType::SampledTexture {
                dimension: wgpu::TextureViewDimension::D3,
                component_type: wgpu::TextureComponentType::Uint,
                multisampled: false,
            },
            wgpu::BindingType::Sampler {
                comparison: false,
            },
        ]
        )
}

fn edit_map_layout(device: &wgpu::Device) 
-> wgpu::BindGroupLayout {
    bind_group_layout_compute(&device, 
        &[
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D3,
                format: wgpu::TextureFormat::R8Uint,
                readonly: false,
            }
        ]
    )
}

fn halve_map_layout(device: &wgpu::Device)
-> wgpu::BindGroupLayout {
    bind_group_layout_compute(&device, 
        &[
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D3,
                format: wgpu::TextureFormat::R8Uint,
                readonly: true,
            },
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D3,
                format: wgpu::TextureFormat::R8Uint,
                readonly: false,
            }
        ]
    )
}

fn edit_sum_table_layout(device: &wgpu::Device) 
-> wgpu::BindGroupLayout {
    bind_group_layout_compute(&device,
        &[
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D3,
                format: wgpu::TextureFormat::R16Uint,
                readonly: false,
            }
        ]
    )
}

fn view_layout(device: &wgpu::Device) 
-> wgpu::BindGroupLayout {
    bind_group_layout_compute(&device, 
        &[
            wgpu::BindingType::UniformBuffer {
                dynamic: false,
                min_binding_size: None,
            }
        ]
    )
}

fn process_layout(device: &wgpu::Device) 
-> wgpu::BindGroupLayout {
    device.create_bind_group_layout(
        & wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &process_entries(),
        }
    )
}

fn depth_shade_layout(device: &wgpu::Device) 
-> wgpu::BindGroupLayout {
    bind_group_layout_compute(&device, 
        &[
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D2,
                format: wgpu::TextureFormat::R32Float,
                readonly: true,
            },
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D2,
                format: wgpu::TextureFormat::Rg11b10Float,
                readonly: false,
            },
        ]
    )
}

fn march_layout(device: &wgpu::Device)
-> wgpu::BindGroupLayout {
    bind_group_layout_compute(&device, 
        &[
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D3,
                format: wgpu::TextureFormat::R16Uint,
                readonly: true,
            },
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D2,
                format: wgpu::TextureFormat::R32Float,
                readonly: false,
            },
            wgpu::BindingType::SampledTexture {
                dimension: wgpu::TextureViewDimension::D3,
                component_type: wgpu::TextureComponentType::Uint,
                multisampled: false,
            },
            wgpu::BindingType::Sampler {
                comparison: false,
            },
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D2,
                format: wgpu::TextureFormat::R32Float,
                readonly: true,
            },
        ]
    )
}

fn map_layout(device: &wgpu::Device) 
-> wgpu::BindGroupLayout {
    bind_group_layout_compute(&device,
        &[
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D3,
                format: wgpu::TextureFormat::R16Uint,
                readonly: true,
            }
        ]
    )
}


fn process_entries()
-> Vec<wgpu::BindGroupLayoutEntry> {
    vec![
        wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStage::FRAGMENT,
            ty: 
                wgpu::BindingType::SampledTexture {
                    dimension: wgpu::TextureViewDimension::D2,
                    component_type: wgpu::TextureComponentType::Float,
                    multisampled: false,
                },
            count: None,
        },
        wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStage::FRAGMENT,
            ty: 
                wgpu::BindingType::Sampler {
                    comparison: false,
                },
            count: None,
        }
    ]
}

fn bind_group_layout_compute(device: &wgpu::Device, binding_types: &[wgpu::BindingType]) 
-> wgpu::BindGroupLayout {

    device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &make_compute_entries(binding_types)
        }
    )
}

fn make_compute_entries(binding_types: &[wgpu::BindingType]) 
-> Vec<wgpu::BindGroupLayoutEntry> {
    binding_types
    .to_vec()
    .into_iter()
    .enumerate()
    .map(|(i, bt)| 
        wgpu::BindGroupLayoutEntry {
            binding: i as u32,
            visibility: wgpu::ShaderStage::COMPUTE,
            ty: bt,
            count: None,
        }
    ).collect()
}