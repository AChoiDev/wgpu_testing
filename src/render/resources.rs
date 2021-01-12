use wgpu::util::DeviceExt;

pub const MONO_BIT_LEVELS: u32 = 4;

pub struct Resources {
    pub render_textures: RenderTextures,
    pub map_texture: wgpu::Texture,
    pub sum_texture: wgpu::Texture,
    pub mono_bit_map_texture: wgpu::Texture,
    pub buffers: Buffers,
    pub default_sampler: wgpu::Sampler,
}

impl Resources {
    pub fn map_texture_copy_view(&self) 
    -> wgpu::TextureCopyView {
            wgpu::TextureCopyView {
                texture: &self.map_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            }
    }

    pub fn new(device: &wgpu::Device) 
    -> Self {
        let render_textures = 
            RenderTextures::new(&device);
        let buffers = 
            Buffers::new(&device);
        let sum_texture = 
            device.create_texture(
                &wgpu::TextureDescriptor {
                    label: None,
                    size: 
                        wgpu::Extent3d {
                            width: 33,
                            height: 33,
                            depth: 33,
                        },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D3,
                    format: wgpu::TextureFormat::R16Uint,
                    usage:
                        wgpu::TextureUsage::STORAGE,
                }
            );

        let map_texture = 
            device.create_texture(
                &wgpu::TextureDescriptor {
                    label: None,
                    size: 
                        wgpu::Extent3d {
                            width: 32,
                            height: 32,
                            depth: 32,
                        },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D3,
                    format: wgpu::TextureFormat::R8Uint,
                    usage:
                        wgpu::TextureUsage::COPY_DST |
                        wgpu::TextureUsage::STORAGE,
                }
            );

        let mono_bit_map_texture =
            device.create_texture(
                &wgpu::TextureDescriptor {
                    label: None,
                    size: 
                        wgpu::Extent3d {
                            width: 16,
                            height: 16,
                            depth: 16,
                        },
                    mip_level_count: MONO_BIT_LEVELS,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D3,
                    format: wgpu::TextureFormat::R8Uint,
                    usage:
                        wgpu::TextureUsage::SAMPLED |
                        wgpu::TextureUsage::STORAGE,
                }
            );

        
        let default_sampler =
            device.create_sampler(
                &wgpu::SamplerDescriptor::default()
            );
   
        Self {
            render_textures,
            buffers,
            map_texture,
            sum_texture,
            mono_bit_map_texture,
            default_sampler,
        }

    }
}

#[allow(dead_code)]
pub struct RenderTextures {
    pub color: wgpu::Texture,
    pub depth: wgpu::Texture,
    pub cone_depth: wgpu::Texture,
}

impl RenderTextures {
    pub fn new(device: &wgpu::Device)
    -> Self {

        let trace_texture_descriptor_base =
            wgpu::TextureDescriptor {
                label: None,
                size: 
                    wgpu::Extent3d {
                        width: crate::RENDER_RES_X,
                        height: crate::RENDER_RES_Y,
                        depth: 1,
                    },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rg11b10Float,
                usage: 
                    wgpu::TextureUsage::STORAGE,
            };

        Self {
            color: 
                device.create_texture(
                    &wgpu::TextureDescriptor {
                        format: wgpu::TextureFormat::Rg11b10Float,
                        usage: wgpu::TextureUsage::STORAGE | wgpu::TextureUsage::SAMPLED,
                        ..trace_texture_descriptor_base
                    }
                ),
            depth:
                device.create_texture(
                    &wgpu::TextureDescriptor {
                        format: wgpu::TextureFormat::R32Float,
                        ..trace_texture_descriptor_base
                    }
                ),               
            cone_depth:
                device.create_texture(
                    &wgpu::TextureDescriptor {
                        format: wgpu::TextureFormat::R32Float,
                        size: 
                            wgpu::Extent3d {
                                width: CONE_DEPTH_RES_X,
                                height: CONE_DEPTH_RES_Y,
                                depth: 1,
                            },
                        ..trace_texture_descriptor_base
                    }
                ),
        }
    }
}

pub const CONE_DEPTH_RES_X: u32 = round_up_div(crate::RENDER_RES_X, 8);
pub const CONE_DEPTH_RES_Y: u32 = round_up_div(crate::RENDER_RES_Y, 8);

pub const fn round_up_div(val: u32, divisor: u32) 
-> u32 {
    (val + divisor - 1) / divisor
}

pub struct Buffers {
    pub screen_quad: wgpu::Buffer,
    pub trace_frame: wgpu::Buffer,
}

impl Buffers {
    pub fn new(device: &wgpu::Device)
    -> Self {

        let quad_verts: [[f32 ; 2] ; 4] = [[-1., -1.], [-1., 1.], [1., -1.], [1., 1.]];

        let screen_quad =
            device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: 
                        bytemuck::bytes_of(&quad_verts),
                    usage: wgpu::BufferUsage::VERTEX,
                },
            );

        let trace_frame =
            device.create_buffer(
                &wgpu::BufferDescriptor {
                    label: None,
                    size: 80,
                    mapped_at_creation: false,
                    usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                },
            );

        Self {
            screen_quad,
            trace_frame,
        }
    }
}