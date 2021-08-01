use crate::{map_3D::Map3D, standard_voxel_prefab::StandardVoxelPrefab};
use nalgebra as na;
use wgpu::Extent3d;

use super::resources::{ChunkIDVariant};


#[allow(dead_code)]
pub struct RenderContext {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,

    surface: wgpu::Surface,
    swapchain: wgpu::SwapChain,

    bind_groups: super::bind_groups::BindGroups,
    pipelines: super::pipelines::Pipelines,
    resources: super::resources::Resources,

    imgui_renderer: super::imgui::ImguiRenderer,
}

impl RenderContext {
    pub fn new(window: &winit::window::Window, partition_count: u32) 
    -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let surface = unsafe { instance.create_surface(window) };
        let prefabs = vec![
            StandardVoxelPrefab::new("resources/bricks.vox"),
            StandardVoxelPrefab::new("resources/inscribed stone.vox"),
            StandardVoxelPrefab::new("resources/ridged_stone.vox")
        ];
        // let voxel_prefab_brick = ;

    let adapter = 
        futures::executor::block_on(
            instance.request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                }
            )
        )
        .expect("Could not create an adapter!");

    let (device, queue) =
        futures::executor::block_on(
            adapter.request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: false,
                },
                None,
            )
        )
        .expect("Could not create device from adapter");
        let swapchain_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::Immediate,
        };
        let swapchain = 
            device.create_swap_chain(&surface, &swapchain_descriptor);

    let bind_group_layouts = super::bind_group_layouts::BindGroupLayouts::new(&device);

        
    let resources = super::resources::Resources::new(&device, partition_count, prefabs.len() as u32);
       
    
    let pipelines = 
        super::pipelines::Pipelines::new(&device, &bind_group_layouts, &swapchain_descriptor);

    let views = super::resource_views::ResourceViews::new(&resources);

    let bind_groups = 
        super::bind_groups::BindGroups::new(
            &device, 
            &bind_group_layouts, 
            &views
        );
    let imgui_renderer =
        super::imgui::ImguiRenderer::new(&queue, &device, &swapchain_descriptor, &window);

        let mut rc = 
        Self {
            instance,
            device,
            adapter,
            queue,
            surface,
            swapchain,
            bind_groups,
            pipelines,
            resources,
            imgui_renderer,
        };

        rc.init_prefabs(prefabs);

        rc
    }


    pub fn init_prefabs(&mut self, prefabs: Vec<StandardVoxelPrefab>) {

        let mega_palette: Vec<u32> = prefabs.iter().flat_map(|p| p.palette.iter()).map(|v| *v).collect();
        // for (i, c) in mega_palette.iter().enumerate() {
            // println!("{}: {:b}", i, c);
        // }

        self.queue.write_texture(
            self.resources.palette_texture_copy_view(),
            unsafe {mega_palette[..].align_to().1},
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 256 * 4,
                rows_per_image: prefabs.len() as u32
            },
            wgpu::Extent3d {
                width: 256,
                height: prefabs.len() as u32,
                depth: 1
            }
        );



        for (i, prefab) in prefabs.into_iter().enumerate() {
            let mut encoder = 
                self.device.create_command_encoder(
                    &wgpu::CommandEncoderDescriptor {
                        label: None,
                    }
            );
            // if i > 0 {
                // continue;
            // }
            self.upload_prefab(&mut encoder, prefab, i as u32);
            self.queue.submit(Some(encoder.finish()));
        }
    }

    pub fn render(&mut self, render_desc: RenderDescriptor) {
        let frame = 
            self.swapchain.get_current_frame()
            .expect("Failed to get next swapchain texture!");


        let mut encoder = 
            self.device.create_command_encoder(
                &wgpu::CommandEncoderDescriptor {
                    label: None,
                }
            );

        // write each map texture chunk that needs to be uploaded
        render_desc.map_data
        .iter()
        .for_each(|m|
            self.upload_index_map(ChunkIDVariant::PartitionID(m.0 as u32), m.1)
        );

        assert!(render_desc.map_data.len() < 2);


        // write layer index map
        self.queue.write_texture(
            self.resources.map_texture_copy_view_reserved(),
            unsafe {render_desc.layer_index_data[..].align_to().1},
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 45 * 2,
                rows_per_image: 15,
            },
            wgpu::Extent3d {
                width: 45,
                height: 15,
                depth: 45,
            }
        );

        // upload once-per-frame view buffers
        {
            let data = super::shader_data::trace_frame::make_bytes(
                render_desc.pos, [crate::RENDER_RES_X, crate::RENDER_RES_Y],
                render_desc.cam_orientation, 100f32, );
            self.queue.write_buffer(&self.resources.buffers.trace_frame, 0, &data);
        }

        if render_desc.map_data.len() > 0 {
            self.construct_bit_volume(&mut encoder,
                super::resources::ChunkIDVariant::PartitionID(render_desc.map_data[0].0 as u32));
        }

        use super::resources::div_ceil;

        let res_dispatch = [div_ceil(crate::RENDER_RES_X, 8), div_ceil(crate::RENDER_RES_Y, 8)];

        // self.cone_trace(&mut encoder);

        {
            let mut cpass = encoder.begin_compute_pass();

            cpass.set_pipeline(&self.pipelines.march);
            cpass.set_bind_group(0, &self.bind_groups.primary, &[]);
            cpass.dispatch(res_dispatch[0], res_dispatch[1], 1);
        }

        {
            let mut cpass = encoder.begin_compute_pass();
            cpass.set_pipeline(&self.pipelines.depth_shade);
            cpass.set_bind_group(0, &self.bind_groups.primary, &[]);
            cpass.dispatch(res_dispatch[0], res_dispatch[1], 1);
        }


        let sc_rpass_desc =
            &wgpu::RenderPassDescriptor {
                color_attachments:
                    &swapchain_only_color_attachments(
                        &frame.output.view
                    ),
                depth_stencil_attachment: None,
            };


        self.copy_to_swapchain_by_screen_quad(&sc_rpass_desc, &mut encoder);

        self.imgui_render(&sc_rpass_desc, render_desc.window, render_desc.delta_time, render_desc.cam_orientation, &mut encoder);

        self.queue.submit(Some(encoder.finish()));

    }

    fn upload_index_map(&self, chunk_id_variant: ChunkIDVariant, map: &Map3D<u16>) {
        self.queue.write_texture(
            self.resources.map_texture_copy_view_chunk_id(chunk_id_variant),
            unsafe {map.full_slice().align_to().1},
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 32 * 2,
                rows_per_image: 32,
            },
            wgpu::Extent3d {
                width: 32,
                height: 32,
                depth: 32,
            }
        )
    }

    pub fn upload_prefab(&self, encoder: &mut wgpu::CommandEncoder, prefab: StandardVoxelPrefab, prefab_id: u32) {
        let variant = ChunkIDVariant::PrefabID(prefab_id);

        self.upload_index_map(variant.clone(), &prefab.palette_volume);
        self.construct_bit_volume(encoder, variant);
    }

    pub fn construct_bit_volume(&self, encoder: &mut wgpu::CommandEncoder, chunk_id_variant: ChunkIDVariant) {
        let chunk_coords = super::resources::chunk_id_to_chunk_coords(
            super::resources::chunk_id_variant_to_id(chunk_id_variant)
        );
        self.queue.write_buffer(
            &self.resources.buffers.chunk_changes,
            0,
            unsafe {chunk_coords.align_to().1}
        );
        // Fill bit volume from initial uploaded map
        {
            let mut cpass = encoder.begin_compute_pass();

            cpass.set_pipeline(&self.pipelines.fill_bit_volume);
            cpass.set_bind_group(0, &self.bind_groups.primary, &[]);
            cpass.set_bind_group(1, &self.bind_groups.edit_mono_bit_map_texture, &[]);
            cpass.dispatch(4, 4, 4);
        }

        [2, 1, 0]
        .iter()
        .map(|&i| 2u32.pow(i))
        .enumerate()
        .for_each(|(i, d)| {
            let mut cpass = encoder.begin_compute_pass();
            cpass.set_pipeline(&self.pipelines.halve_bit_volume);
            cpass.set_bind_group(0, &self.bind_groups.halve_map_binds[i], &[]);
            cpass.set_bind_group(1, &self.bind_groups.edit_mono_bit_map_texture, &[]);
            cpass.dispatch(d, d, d);
        });
    }

    pub fn cone_trace(&self, encoder: &mut wgpu::CommandEncoder) {
        let mut cpass = encoder.begin_compute_pass();
        cpass.set_pipeline(&self.pipelines.cone_march);
        cpass.set_bind_group(0, &self.bind_groups.primary, &[]);
        cpass.dispatch((super::resources::CONE_DEPTH_RES_X + 7) / 8, (super::resources::CONE_DEPTH_RES_Y + 7) / 8, 1);
    }

    pub fn copy_to_swapchain_by_screen_quad(&self, 
        sc_rpass_desc: &wgpu::RenderPassDescriptor,
        encoder: &mut wgpu::CommandEncoder) 
    {
        let mut rpass = encoder.begin_render_pass(sc_rpass_desc);

        rpass.set_pipeline(&self.pipelines.process);
        rpass.set_bind_group(0, &self.bind_groups.primary, &[]);
        rpass.set_vertex_buffer(0, self.resources.buffers.screen_quad.slice(..));
        rpass.draw(0..4, 0..1);
    }

    pub fn imgui_render(&mut self,
        sc_rpass_desc: &wgpu::RenderPassDescriptor,
        window: &winit::window::Window,
        delta_time: f32,
        cam_orientation: na::UnitQuaternion<f32>,
        encoder: &mut wgpu::CommandEncoder) 
    {
        let rpass = encoder.begin_render_pass(sc_rpass_desc);
        self.imgui_renderer.render(&self.device, &self.queue, rpass, delta_time, cam_orientation, window);
    }
}


fn swapchain_only_color_attachments(
    swapchain_view: &wgpu::TextureView) 
-> [wgpu::RenderPassColorAttachmentDescriptor ; 1] {
    [
        wgpu::RenderPassColorAttachmentDescriptor {
            attachment: swapchain_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: true,
            },
        }
    ]
}



pub struct RenderDescriptor<'a> {
    pub window: &'a winit::window::Window,
    pub map_data: Vec<(usize, &'a Map3D<u16>)>,
    pub layer_index_data: Vec<u16>,
    pub cam_orientation: na::UnitQuaternion<f32>,
    pub pos: na::Vector3<f32>,
    pub delta_time: f32,
    pub frame: u32,
}