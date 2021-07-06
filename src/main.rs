use std::iter::Map;

use displaced_chunks::DisplacedChunks;
use map_3D::Map3D;
use render::render_context::RenderDescriptor;
//use octree_texture::OctreeTexture;

pub const WINDOW_X: u32 = 1920 / 2;
pub const WINDOW_Y: u32 = 1080 / 2;
pub const RENDER_RES_X: u32 = 480;
pub const RENDER_RES_Y: u32 = 270;
mod map_3D;
mod render;
mod displaced_chunks;
mod octree_texture;

use nalgebra as na;
use octree_texture::OctreeTexture;

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

    {
        let mut  map = map_3D::Map3D::<u8>::new(32);

        map.set_all(&(|c| fill_voxel(c, 0)));

        let tree = octree_texture::OctreeTexture::new_from_map(&map, 5);

        let mut mistake = false;
        'outer: for x in 0..32 {
        for y in 0..32 {
        for z in 0..32 {
            let coords = [x, y, z];
            if (map.get(coords) == 0) != tree.get(coords) {
                mistake = true;
                break 'outer;
            }
        }
        }
        }

        println!("mistake: {}", mistake);
        println!("node count: {}", tree.total_nodes());
    }

    //let mut map_grid = map_3D::Map3D::new(32);

    let mut displaced_chunks = DisplacedChunks::<map_3D::Map3D<u8>>::new();

    let mut frame_count = 0u32;
    let mut frame_time = std::time::Instant::now();

    let mut orientation = na::UnitQuaternion::<f32>::identity();
    let mut pos = na::Vector3::repeat(15f32);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;

        match event {
            winit::event::Event::MainEventsCleared => {
                displaced_chunks.try_initialize(na::zero());

                window.request_redraw();
            },
            winit::event::Event::RedrawRequested(_window_id) => {

                let delta_time = frame_time.elapsed().as_secs_f32();
                frame_time = std::time::Instant::now();

                let displacement_index_map_data = displaced_chunks.get_index_map();

                render_context.render(
                    RenderDescriptor {
                        window: &window,
                        cam_orientation: orientation,
                        map_data: displaced_chunks.clean_dirty_chunks(),
                        displacement_index_map_data,
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
                pos += 0.1f32 * orientation.transform_vector(&na::Vector3::z());
            }
            if input.key_held(winit::event::VirtualKeyCode::E) {
                pos -= 0.1f32 * orientation.transform_vector(&na::Vector3::z());
            }

            if input.key_pressed(winit::event::VirtualKeyCode::Space) {
                displaced_chunks.displace(na::Vector3::new(1, 0, 0));
            }

            let mouse_diff = input.mouse_diff();

            orientation = na::UnitQuaternion::from_euler_angles(0., mouse_diff.0 * 0.002, 0.) * orientation;
            orientation *= na::UnitQuaternion::from_euler_angles(-mouse_diff.1 * 0.002, 0.0, 0.0);
        }

    });
}

pub fn fill_voxel(coords: [usize ; 3], frame: u32) 
-> u8 
{
    let disp = [15 - coords[0] as i32, 15 - coords[1] as i32, 15 - coords[2] as i32];
    let mag_sqr = disp[0] * disp[0] + disp[1] * disp[1] + disp[2] * disp[2];

    if mag_sqr < 19i32.pow(2) && ((frame / 20) % 2 == 0 || coords[1] < 25) {
        255 // not filled
    } 
    else {
        0 // filled
    }
}


