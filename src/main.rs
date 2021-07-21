use displaced_chunks::DisplacedChunks;
use render::render_context::RenderDescriptor;

pub const WINDOW_X: u32 = 1920;
pub const WINDOW_Y: u32 = 1080;
pub const RENDER_RES_X: u32 = 480 * 3;
pub const RENDER_RES_Y: u32 = 270 * 3;
mod map_3D;
mod render;
mod displaced_chunks;
mod dot_vox_wrapper;
mod bit_voxels;
mod standard_voxel_prefab;

use nalgebra as na;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();

    let window =
        winit::window::WindowBuilder::new()
        .with_title("Voxel Testing")
        .with_inner_size(winit::dpi::PhysicalSize::new(WINDOW_X, WINDOW_Y))
        .build(&event_loop)
        .unwrap();

    let mut render_context = render::render_context::RenderContext::new(&window);

    let mut input = winit_input_helper::WinitInputHelper::new();

    //let mut map_grid = map_3D::Map3D::new(32);
    let mut view_partition_coords = na::Vector3::new(-121, 0, 412);
    let mut displaced_chunks = DisplacedChunks::<map_3D::Map3D<u16>>::new(view_partition_coords);

    let mut frame_count = 0u32;
    let mut frame_time = std::time::Instant::now();
    let mut delta_time = 0f32;

    let mut orientation = na::UnitQuaternion::<f32>::identity();
    let mut pos = na::Vector3::repeat(15f32);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;

        // window.set_cursor_position(winit::dpi::LogicalPosition {x: 1, y: 1}).unwrap();

        match event {
            winit::event::Event::MainEventsCleared => {

                window.request_redraw();
            },
            winit::event::Event::RedrawRequested(_window_id) => {

                displaced_chunks.try_initialize();

                delta_time = frame_time.elapsed().as_secs_f32();
                frame_time = std::time::Instant::now();

                let layer_index_data = displaced_chunks.get_index_map();
                let map_data = displaced_chunks.clean_dirty_chunks();

                render_context.render(
                    RenderDescriptor {
                        window: &window,
                        cam_orientation: orientation,
                        map_data,
                        layer_index_data,
                        pos,
                        delta_time,
                        frame: frame_count
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
            if input.key_held(winit::event::VirtualKeyCode::Period) {
                pos += 100f32 * orientation.transform_vector(&na::Vector3::z()) * delta_time;
            }
            if input.key_held(winit::event::VirtualKeyCode::E) {
                pos -= 100f32 * orientation.transform_vector(&na::Vector3::z()) * delta_time;
            }
            if input.key_pressed(winit::event::VirtualKeyCode::P) {
                view_partition_coords += na::Vector3::new(1, 0, 0);
                displaced_chunks.set_view_partition_coords(view_partition_coords);
                // displaced_chunks.displace(na::Vector3::new(1, 0, 0));

            }

            let mouse_diff = input.mouse_diff();

            orientation = na::UnitQuaternion::from_euler_angles(0., mouse_diff.0 * 0.002, 0.) * orientation;
            orientation *= na::UnitQuaternion::from_euler_angles(-mouse_diff.1 * 0.002, 0.0, 0.0);
        }

    });
}

pub fn fill_voxel(coords: [usize ; 3], partition_coords: na::Vector3<i32>) 
-> u16 
{
    let disp = [15 - coords[0] as i32, 15 - coords[1] as i32, 15 - coords[2] as i32];
    let mag_sqr = disp[0] * disp[0] + disp[1] * disp[1] + disp[2] * disp[2];

    let is_in_sphere = mag_sqr < 20i32.pow(2);

    // if coords == [0, 0, 0] {
        // println!("partition_coords: {}", partition_coords);
    // }

    if partition_coords.y < 0 && coords == [15, 15, 15] {
        return 0;
    }
    if partition_coords.y < 0 {
        return u16::MAX;
    }

    if partition_coords.x.rem_euclid(4) == 0 {
        if is_in_sphere {
            0
        } else {
            u16::MAX
        }
    } else if partition_coords.x.rem_euclid(4) == 1 {
        if is_in_sphere {
            u16::MAX
        } else {
            0
        }
    } else if partition_coords.x.rem_euclid(4) == 2 {
        0
    } else {
        if (coords[0] / 4 + coords[1] / 4  + coords[2] / 4) % 2 != 0 && !is_in_sphere {
            0
        } else {
            u16::MAX
        }
    }
}

