use imgui::im_str;
use nalgebra as na;

const FRAME_AVG_COUNT: usize = 20;

pub struct ImguiRenderer {
    context: imgui::Context,
    wgpu_renderer: imgui_wgpu::Renderer,
    platform: imgui_winit_support::WinitPlatform,
    accumulated_deltas: std::collections::VecDeque<f32>,
}

impl ImguiRenderer {
    pub fn new(
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        swapchain_descriptor: &wgpu::SwapChainDescriptor,
        window: &winit::window::Window)
    -> Self {
        let mut context = imgui::Context::create();
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut context);
        platform.attach_window(context.io_mut(), &window, imgui_winit_support::HiDpiMode::Default);
        context.set_ini_filename(None);

        context.io_mut().font_global_scale = (1. / window.scale_factor()) as f32;

        context.fonts().add_font(
            &[
                imgui::FontSource::DefaultFontData {
                    config: 
                        Some(
                            imgui::FontConfig {
                                oversample_h: 1,
                                pixel_snap_h: true,
                                size_pixels: (13.0 * window.scale_factor()) as f32,
                                ..Default::default()
                            }
                        )
                },
            ]
        );

        let wgpu_renderer = 
            imgui_wgpu::Renderer::new(
                &mut context,
                device,
                queue,
                imgui_wgpu::RendererConfig {
                    texture_format: swapchain_descriptor.format,
                    ..Default::default()
                },
            );
        Self {
            context,
            wgpu_renderer,
            platform,
            accumulated_deltas: std::collections::VecDeque::new(),
        }
    }

    pub fn render<'a>(
        &'a mut self, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue, 
        mut rpass: wgpu::RenderPass<'a>, 
        delta_time: f32,
        orientation: na::UnitQuaternion<f32>,
        window: &winit::window::Window) 
    {
        self.accumulated_deltas.truncate(FRAME_AVG_COUNT);
        self.accumulated_deltas.push_front(delta_time);
        let avg_delta = 
            self.accumulated_deltas.iter().fold(0., |iv, a| iv + a) 
            / self.accumulated_deltas.len() as f32;

        self.context.io_mut()
        .update_delta_time(std::time::Duration::from_secs_f32(delta_time));

        self.platform
        .prepare_frame(self.context.io_mut(), &window)
        .expect("Failed to prepare imgui frame");

        let ui = self.context.frame();

        {
            let imgui_window = imgui::Window::new(im_str!("Frame Timings"));

            imgui_window
            .size([200., 120.], imgui::Condition::FirstUseEver)
            .position([20., 20.], imgui::Condition::FirstUseEver)
            .build(
                &ui,
                || {
                    ui.text(
                        im_str!(
                            "Delta Time: {:.3} ms",
                            avg_delta * 1000.0
                        )
                    );
                    ui.text(
                        im_str!(
                            "Frame Rate: {:.1} Hz",
                            1.0 / avg_delta
                        )
                    );
                    ui.separator();
                    
                    ui.text_wrapped(&im_str!(
                        "Timings are taken as an average over the past {} frames",
                        FRAME_AVG_COUNT,
                    ));
                }
            );
        }

        {
            let imgui_window = imgui::Window::new(im_str!("View Information"));

            let forward = 
                orientation.transform_vector(&na::Vector3::z_axis()).normalize();
            let right = 
                orientation.transform_vector(&na::Vector3::x_axis()).normalize();
            let up = 
                orientation.transform_vector(&na::Vector3::y_axis()).normalize();

            imgui_window
            .size([300., 120.], imgui::Condition::FirstUseEver)
            .position([20., 400.], imgui::Condition::FirstUseEver)
            .build(
                &ui,
                || {
                    ui.text(im_str!(
                        "Forward: ({:.3}, {:.3}, {:.3})", 
                        forward.x, forward.y, forward.z
                    ));
                    ui.text(im_str!(
                        "up: ({:.3}, {:.3}, {:.3})", 
                        up.x, up.y, up.z
                    ));
                    ui.text(im_str!(
                        "right: ({:.3}, {:.3}, {:.3})", 
                        right.x, right.y, right.z
                    ));
                }
            )
        }

        self.platform.prepare_render(&ui, &window);

        self.wgpu_renderer
        .render(ui.render(), &queue, &device, &mut rpass)
        .expect("wgpu imgui render failed");
    }
}
