use crate::map_3D::Map3D;
use nalgebra as na;


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
    pub fn new(window: &winit::window::Window) 
    -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let surface = unsafe { instance.create_surface(window) };

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

        
    let resources = super::resources::Resources::new(&device);
       
    
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

        let octrees: Vec<_> =
            render_desc.map_data
            .iter()
            .map(|m|
                (m.0, crate::octree_texture::OctreeTexture::new_from_map(m.1, 5))
            ).collect();

        octrees
        .iter()
        .for_each(|m|
            self.queue.write_texture(
                self.resources.oct_map_texture_copy_view_by_index(m.0 as u32),
                bytemuck::cast_slice(m.1.full_slice()),
                wgpu::TextureDataLayout {
                    offset: 0,
                    bytes_per_row: 36,
                    rows_per_image: 18,
                },
                wgpu::Extent3d {
                    width: 18,
                    height: 18,
                    depth: 18,
                }
            )
        );

        // upload displacement index map
        self.queue.write_texture(
            self.resources.displacement_index_map_copy_view(),
            unsafe { render_desc.displacement_index_map_data.full_slice().align_to::<u8>().1 },
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 26,
                rows_per_image: 13,
            },
            wgpu::Extent3d {
                width: 13, height: 13, depth: 13,
            }
        );

        // upload view buffers
        {
            let data = super::shader_data::trace_frame::make_bytes(
                render_desc.pos, [crate::RENDER_RES_X, crate::RENDER_RES_Y],
                render_desc.cam_orientation, 100f32, );
            self.queue.write_buffer(&self.resources.buffers.trace_frame, 0, &data);
        }


        use super::resources::div_ceil;

        let res_dispatch = [div_ceil(crate::RENDER_RES_X, 8), div_ceil(crate::RENDER_RES_Y, 8)];

        {
            let mut cpass = encoder.begin_compute_pass();
            cpass.set_pipeline(&self.pipelines.octree_march);
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
    pub map_data: Vec<(usize, &'a Map3D<u8>)>,
    pub displacement_index_map_data: Map3D<u16>,
    pub cam_orientation: na::UnitQuaternion<f32>,
    pub pos: na::Vector3<f32>,
    pub delta_time: f32,
    pub frame: u32,
}