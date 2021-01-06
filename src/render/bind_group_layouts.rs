
pub struct BindGroupLayouts {
    pub process: wgpu::BindGroupLayout,
    pub march: wgpu::BindGroupLayout,
    pub edit_sum_table: wgpu::BindGroupLayout,
}

impl BindGroupLayouts {
    pub fn new(device: &wgpu::Device)
    -> Self {
        Self {
            march:
                device.create_bind_group_layout(
                    &wgpu::BindGroupLayoutDescriptor {
                        label: None,
                        entries: &make_trace_compute_entries(),
                    } 
                ),
            process:
                device.create_bind_group_layout(
                    &wgpu::BindGroupLayoutDescriptor {
                        label: None,
                        entries: &make_process_compute_entries(),
                            
                    }
               ),
            edit_sum_table:
                device.create_bind_group_layout(
                    &wgpu::BindGroupLayoutDescriptor {
                        label: None,
                        entries: &make_process_sum_entries(),
                    }
               ),
               
        }
    }
}

fn make_process_sum_entries()
-> Vec<wgpu::BindGroupLayoutEntry> {
    make_compute_entries(
        vec![
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D3,
                format: wgpu::TextureFormat::R8Uint,
                readonly: true,
            },
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D3,
                format: wgpu::TextureFormat::R16Uint,
                readonly: false,
            },
        ]
    )
}

fn make_process_compute_entries()
-> Vec<wgpu::BindGroupLayoutEntry> {
    vec![
        wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStage::FRAGMENT,
            ty: 
                wgpu::BindingType::StorageTexture {
                    dimension: wgpu::TextureViewDimension::D2,
                    format: wgpu::TextureFormat::Rg11b10Float,
                    readonly: true,
                },
            count: None,
        }
    ]
}

fn make_trace_compute_entries() 
-> Vec<wgpu::BindGroupLayoutEntry> {
    make_compute_entries(
        vec![
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D2,
                format: wgpu::TextureFormat::Rg11b10Float,
                readonly: false,
            },
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D3,
                format: wgpu::TextureFormat::R8Uint,
                readonly: true,
            },
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D3,
                format: wgpu::TextureFormat::R16Uint,
                readonly: true,
            },
            wgpu::BindingType::UniformBuffer {
                dynamic: false,
                min_binding_size: None,
            },
        ]
    )
}

fn make_compute_entries(binding_types: Vec<wgpu::BindingType>) 
-> Vec<wgpu::BindGroupLayoutEntry> {
    binding_types
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