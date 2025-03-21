use wgpu::util::DeviceExt;

pub const MONO_BIT_LEVELS: u32 = 4;

pub struct Resources {
    pub render_textures: RenderTextures,
    pub map_texture: wgpu::Texture,
    pub mono_bit_map_texture: wgpu::Texture,
    pub buffers: Buffers,
    pub default_sampler: wgpu::Sampler,
    pub palette_array: wgpu::Texture,
}

// a unique id for a chunk in texture memory
#[derive(Clone)]
pub enum ChunkIDVariant {
    PrefabID(u32),
    PartitionID(u32),
    LayerID(u32),
}

pub fn chunk_id_to_chunk_coords(chunk_id: u32) -> [u32 ; 3] {
    // the curve moves across the 3D array like row major order
    // except its x-axis then z-axis then y-axis
    [chunk_id % 32, chunk_id / (32 * 32), (chunk_id / 32) % 32]
}

pub fn chunk_id_variant_to_id(chunk_id_variant: ChunkIDVariant) -> u32 {
    const MAX_PREFAB_IDS: u32 = 32 * 32 * 2;
    const MAX_LAYER_IDS: u32 = 32 * 2;

    match chunk_id_variant {
        ChunkIDVariant::PrefabID(id) => id,
        ChunkIDVariant::PartitionID(id) => id + MAX_PREFAB_IDS + MAX_LAYER_IDS,
        ChunkIDVariant::LayerID(id) => id + MAX_PREFAB_IDS
    }
}


impl Resources {
    pub fn map_texture_copy_view_chunk_id(&self, chunk_id: ChunkIDVariant)
    -> wgpu::TextureCopyView {
        let chunk_coords = chunk_id_to_chunk_coords(chunk_id_variant_to_id(chunk_id));
        self.map_texture_copy_view(chunk_coords)
    }

    fn map_texture_copy_view(&self, chunk_offset: [u32 ; 3]) 
    -> wgpu::TextureCopyView {
            wgpu::TextureCopyView {
                texture: &self.map_texture,
                mip_level: 0,
                origin: 
                    wgpu::Origin3d {
                        x: 32 * chunk_offset[0],
                        y: 32 * chunk_offset[1],
                        z: 32 * chunk_offset[2],
                    },
            }
    }

    pub fn palette_texture_copy_view(&self) 
    -> wgpu::TextureCopyView {
        wgpu::TextureCopyView {
            texture: &self.palette_array,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        }
    }

    pub fn map_texture_copy_view_reserved(&self) 
    -> wgpu::TextureCopyView {
        wgpu::TextureCopyView {
            texture: &self.map_texture,
            mip_level: 0,
            origin:
                wgpu::Origin3d {
                    x: 0,
                    y: 32 * 2,
                    z: 0,
                },
        }
    }


    pub fn div_ceil_i32(dividend: i32, divisor: i32) -> i32 {
        (dividend + divisor - 1) / divisor
    }

    pub fn new(device: &wgpu::Device, partition_count: u32, palette_count: u32) 
    -> Self {
        let render_textures = 
            RenderTextures::new(&device);
        let buffers = 
            Buffers::new(&device);

        let default_sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

        // let chunk_height = (partition_count as i32 - 960).div
        let chunk_height = (Self::div_ceil_i32(partition_count as i32 - 960, 1024) + 3) as u32;
        println!("chunk_height: {}", chunk_height);

        let map_texture_extents = 
            wgpu::Extent3d {
                width: 32 * 32,
                height: 32 * chunk_height,
                depth: 32 * 32,
            };
   
        Self {
            render_textures,
            buffers,
            map_texture: create_index_map(&device, map_texture_extents),
            mono_bit_map_texture: create_mono_bit_map(&device, chunk_height),
            default_sampler,
            palette_array: create_palette_array(device, palette_count),
        }

    }
}

pub fn create_palette_array(device: &wgpu::Device, palette_count: u32) -> wgpu::Texture {
    device.create_texture(
        &wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: 256,
                height: palette_count,
                depth: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED,
        }
    )
}

pub fn create_index_map(device: &wgpu::Device, extent: wgpu::Extent3d) -> wgpu::Texture {
    device.create_texture(
        &wgpu::TextureDescriptor {
            label: None,
            size: extent,
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

pub fn create_mono_bit_map(device: &wgpu::Device, chunk_height: u32) -> wgpu::Texture {
    device.create_texture(
        &wgpu::TextureDescriptor {
            label: None,
            size: 
                wgpu::Extent3d {
                    width: 16 * 32,
                    height: 16 * chunk_height,
                    depth: 16 * 32,
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
    pub chunk_changes: wgpu::Buffer,
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

        // currently support 6 chunks to be changed in a frame
        // allow more later
        let chunk_changes = 
            device.create_buffer(
                &wgpu::BufferDescriptor {
                    label: None,
                    size: 4 * 6,
                    mapped_at_creation: false,
                    usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                },
            );

        Self {
            screen_quad,
            trace_frame,
            chunk_changes
        }
    }
}