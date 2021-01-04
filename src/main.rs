use render::render_context::RenderDescriptor;

pub const WINDOW_SIZE: u32 = 1024;

mod byte_grid;
mod render;

use nalgebra as na;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();

    let window =
        winit::window::WindowBuilder::new()
        .with_title("Voxel Testing")
        .with_inner_size(winit::dpi::PhysicalSize::new(WINDOW_SIZE, WINDOW_SIZE))
        .build(&event_loop)
        .unwrap();
        

    let mut render_context = render::render_context::RenderContext::new(&window);

    let mut input = winit_input_helper::WinitInputHelper::new();

    let mut map_grid = byte_grid::ByteGrid::new(32);

    let mut frame_count = 0u32;
    let mut frame_time = std::time::Instant::now();

    let mut orientation = na::UnitQuaternion::<f32>::identity();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;

        match event {
            winit::event::Event::MainEventsCleared => {
                window.request_redraw();
            },
            winit::event::Event::RedrawRequested(_window_id) => {

                let delta_time = frame_time.elapsed().as_secs_f32();
                frame_time = std::time::Instant::now();

                map_grid.set_all(
                    &(|coords| fill_voxel(coords, frame_count))
                );

                render_context.render(
                    RenderDescriptor {
                        window: &window,
                        map_data: &map_grid,
                        cam_orientation: orientation,
                        delta_time
                    }
                );

                frame_count += 1;
            },
            _ => {},
        }


        if input.update(&event) {
            if input.key_pressed(winit::event::VirtualKeyCode::Escape) {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }

            let mouse_diff = input.mouse_diff();

            orientation = na::UnitQuaternion::from_euler_angles(0., mouse_diff.0 * 0.002, 0.) * orientation;
            orientation *= na::UnitQuaternion::from_euler_angles(-mouse_diff.1 * 0.002, 0.0, 0.0);
        }

    });
}

fn fill_voxel(coords: [usize ; 3], frame: u32) 
-> u8 
{
    let disp = [15 - coords[0] as i32, 15 - coords[1] as i32, 15 - coords[2] as i32];
    let mag_sqr = disp[0] * disp[0] + disp[1] * disp[1] + disp[2] * disp[2];

    if mag_sqr < 18i32.pow(2) && ((frame / 20) % 2 == 0 || 31 - coords[1] > 6) {
        255 // not filled
    } 
    else {
        0 // filled
    }
}
