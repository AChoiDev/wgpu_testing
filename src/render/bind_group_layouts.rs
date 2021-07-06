

pub struct BindGroupLayouts {
    pub primary_layout: wgpu::BindGroupLayout,
    pub halve_map: wgpu::BindGroupLayout,
    pub edit_map: wgpu::BindGroupLayout,
    pub upload_map: wgpu::BindGroupLayout,
}

impl BindGroupLayouts {
    pub fn new(device: &wgpu::Device)
    -> Self {
        Self {
            primary_layout: create_primary_layout(&device),
            halve_map: halve_map_layout(&device),
            edit_map: edit_map_layout(&device),
            upload_map: upload_map_layout(&device),
        }
    }
}

fn depth_storage_texture() -> wgpu::BindingType {
    wgpu::BindingType::StorageTexture {
        dimension: wgpu::TextureViewDimension::D2,
        format: wgpu::TextureFormat::R32Float,
        readonly: false,
    }
}

fn byte_map_storage_texture(readonly: bool) -> wgpu::BindingType {
    wgpu::BindingType::StorageTexture {
        dimension: wgpu::TextureViewDimension::D3,
        format: wgpu::TextureFormat::R8Uint,
        readonly,
    }
}


fn create_primary_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let binding_types = 
        vec![
            // regular depth
            depth_storage_texture(),

            // main color texture
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D2,
                format: wgpu::TextureFormat::Rg11b10Float,
                readonly: false,
            },
            // view buffer
            wgpu::BindingType::UniformBuffer {
                dynamic: false,
                min_binding_size: None,
            },
            
            // octree map
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D3,
                format: wgpu::TextureFormat::R32Uint,
                readonly: true,
            },
            // index map
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D3,
                format: wgpu::TextureFormat::R16Uint,
                readonly: true,
            },

        ];
    
    device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries:
                &make_entries_visible(
                    make_compute_entries(&binding_types),
                    &[0, 1],
                    wgpu::ShaderStage::COMPUTE | wgpu::ShaderStage::FRAGMENT
                )
        }
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

fn upload_map_layout(device: &wgpu::Device) 
-> wgpu::BindGroupLayout {
    bind_group_layout_compute(&device, 
        &[
            // wgpu::BindingType::UniformBuffer {
                // dynamic: false,
                // min_binding_size: None,
            // },
            wgpu::BindingType::StorageBuffer {
                dynamic: false,
                min_binding_size: None,
                readonly: false,
            },
            wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D3,
                format: wgpu::TextureFormat::R8Uint,
                readonly: false,
            },
        ]
    )
}

fn halve_map_layout(device: &wgpu::Device)
-> wgpu::BindGroupLayout {
    bind_group_layout_compute(&device, 
        &[
            byte_map_storage_texture(true),
            byte_map_storage_texture(false),
        ]
    )
}





fn make_entries_visible(entries: Vec<wgpu::BindGroupLayoutEntry>, indices: &[usize], visibility: wgpu::ShaderStage) 
-> Vec<wgpu::BindGroupLayoutEntry> {

    let mut new_entries = entries.clone();
    
    indices
    .iter()
    .for_each(|&i|
        new_entries[i].visibility = visibility
    );

    new_entries
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