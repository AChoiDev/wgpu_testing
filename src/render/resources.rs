use wgpu::util::DeviceExt;

pub const MONO_BIT_LEVELS: u32 = 4;

pub struct Resources {
    pub render_textures: RenderTextures,
    pub map_texture: wgpu::Texture,
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

        let default_sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());
   
        Self {
            render_textures,
            buffers,
            map_texture: create_map(&device),
            mono_bit_map_texture: create_mono_bit_map(&device),
            default_sampler,
        }

    }
}

pub fn create_map(device: &wgpu::Device) -> wgpu::Texture {
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
    )
}

pub fn create_mono_bit_map(device: &wgpu::Device) -> wgpu::Texture {
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
    )
}

#[allow(dead_code)]
pub struct RenderTextures {
    pub color: wgpu::Texture,
    pub depth: wgpu::Texture,
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

        let depth_res = depth_res([crate::RENDER_RES_X, crate::RENDER_RES_Y]);

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
                        size: 
                            wgpu::Extent3d {
                                width: depth_res[0],
                                height: depth_res[1],
                                depth: 1,
                            },
                        ..trace_texture_descriptor_base
                    }
                ),               
        }
    }
}

fn depth_res(res: [u32 ; 2]) 
-> [u32 ; 2] {
    let cone_res = div_ceil_res(res, CONE_DEPTH_SCALE);
    let pixels = cone_res[0] * cone_res[1];
    let delta_y = div_ceil(pixels, res[0]);

    [res[0], res[1] + delta_y]
}

pub const CONE_DEPTH_SCALE: u32 = 8;


pub const CONE_DEPTH_RES_X: u32 = div_ceil(crate::RENDER_RES_X, 8);
pub const CONE_DEPTH_RES_Y: u32 = div_ceil(crate::RENDER_RES_Y, 8);

pub fn div_ceil_res(val: [u32 ; 2], divisor: u32) -> [u32 ; 2] {
    [div_ceil(val[0], divisor), div_ceil(val[1], divisor)]
}

pub const fn div_ceil(val: u32, divisor: u32) 
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
                    size: 96,
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