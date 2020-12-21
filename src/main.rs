const WINDOW_SIZE: u32 = 256;

fn main() {

    let event_loop = winit::event_loop::EventLoop::new();

    let wgpu_instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

    let window =
        winit::window::WindowBuilder::new()
        .with_title("Voxel Testing")
        .with_inner_size(winit::dpi::PhysicalSize::new(WINDOW_SIZE, WINDOW_SIZE))
        .build(&event_loop)
        .unwrap();


    let mut input = winit_input_helper::WinitInputHelper::new();

    let surface = unsafe { wgpu_instance.create_surface(&window) };

    let sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: wgpu::PresentMode::Immediate,
    };

    let adapter = 
        futures::executor::block_on(
            wgpu_instance.request_adapter(
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

    let gradient_module = 
        device.create_shader_module(
            wgpu::include_spirv!("spirv/gradient.comp.spv")
        );

    let compute_bind_group_layout =     
        device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries:
                    &[
                    ]
            } 
        );

    let compute_pipeline_layout = 
        device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts:
                    &[
                        &compute_bind_group_layout,
                    ],
                push_constant_ranges: &[],
            }
        );

    let compute_pipeline = device.create_compute_pipeline(
        &wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&compute_pipeline_layout),
            compute_stage: wgpu::ProgrammableStageDescriptor {
                module: &gradient_module,
                entry_point: "main",
            },
        }
    );

    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);


    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;

        match event {
            winit::event::Event::RedrawRequested(_window_id) => {
                let frame = 
                    swap_chain.get_current_frame()
                    .expect("Failed to get next swapchain texture!");

                let mut encoder = 
                    device.create_command_encoder(
                        &wgpu::CommandEncoderDescriptor {
                            label: None,
                        }
                    );

                

                {
                    let _rpass = encoder.begin_render_pass(
                        &wgpu::RenderPassDescriptor {
                            color_attachments: &[
                                wgpu::RenderPassColorAttachmentDescriptor {
                                    attachment: &frame.output.view,
                                    resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Clear(
                                            wgpu::Color {
                                                r: 0.5,
                                                g: 0.5,
                                                b: 0.5,
                                                a: 1.0,
                                            },
                                        ),
                                        store: true,
                                    }
                                },
                            ],
                            depth_stencil_attachment: None,
                        }
                    );
                }

                queue.submit(Some(encoder.finish()));
                
            }

            _ => {},
        }

        if input.update(&event) {
            if input.key_pressed(winit::event::VirtualKeyCode::Escape) {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
        }
    });
    
}