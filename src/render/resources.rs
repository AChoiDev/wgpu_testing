use wgpu::util::DeviceExt;


const CHUNKS_WIDTH: u32 = 13;

//const

pub struct Resources {
    pub render_textures: RenderTextures,
    pub oct_map_texture: wgpu::Texture,
    pub buffers: Buffers,
    pub default_sampler: wgpu::Sampler,
    pub displacement_index_map: wgpu::Texture,
}

impl Resources {
    pub fn coords(index: u32, length: u32) 
    -> [u32 ; 3] {
        [
            index % length,
            (index / length) % length,
            index / length.pow(2),
        ]
    }
    

    fn oct_map_texture_copy_view(&self, chunk_offset: [u32 ; 3]) 
    -> wgpu::TextureCopyView {
        let octree_chunk_size = crate::octree_texture::OCTUPLE_DATA_MAP_SIZE as u32;

        wgpu::TextureCopyView {
            texture: &self.oct_map_texture,
            mip_level: 0,
            origin:
                wgpu::Origin3d {
                    x: octree_chunk_size * chunk_offset[0],
                    y: octree_chunk_size * chunk_offset[1],
                    z: octree_chunk_size * chunk_offset[2],
                },
        }
    }

    pub fn oct_map_texture_copy_view_by_index(&self, index: u32)
    -> wgpu::TextureCopyView {
        let chunk_offset = Resources::coords(index, CHUNKS_WIDTH);
        self.oct_map_texture_copy_view(chunk_offset)
    }

    pub fn displacement_index_map_copy_view(&self) 
    -> wgpu::TextureCopyView {
        wgpu::TextureCopyView {
            texture: &self.displacement_index_map,
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
            oct_map_texture: create_oct_map(&device),
            displacement_index_map: create_displacement_index_map(&device),
            default_sampler,
        }

    }
}

pub fn create_displacement_index_map(device: &wgpu::Device) -> wgpu::Texture {
    device.create_texture(
        &wgpu::TextureDescriptor {
            label: None,
            size: 
                wgpu::Extent3d {
                    width: CHUNKS_WIDTH,
                    height: CHUNKS_WIDTH,
                    depth: CHUNKS_WIDTH,
                },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::R16Uint,
            usage:
                wgpu::TextureUsage::COPY_DST |
                wgpu::TextureUsage::STORAGE,
        }
    )
}



pub fn create_oct_map(device: &wgpu::Device) -> wgpu::Texture {
    let octree_chunk_size = crate::octree_texture::OCTUPLE_DATA_MAP_SIZE as u32;
    let width = octree_chunk_size * CHUNKS_WIDTH;
    device.create_texture(
        &wgpu::TextureDescriptor {
            label: None,
            size:
                wgpu::Extent3d {
                    width,
                    height: width,
                    depth: width
                },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::R16Uint,
            usage: wgpu::TextureUsage::STORAGE | wgpu::TextureUsage::COPY_DST,
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