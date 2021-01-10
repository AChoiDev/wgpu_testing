
use nalgebra as na;

#[derive(Default, Copy, Clone)]
#[allow(dead_code)]
struct TraceFrameData {
    pos: [f32 ; 3],
        p0: i32,
    right: [f32 ; 3],
        p1: i32,
    up: [f32 ; 3],
        p2: i32,
    forward: [f32 ; 3],
        p3: i32,
    cotan_half_fov: f32,
        p4: [i32 ; 3],
}
unsafe impl bytemuck::Zeroable for TraceFrameData {}
unsafe impl bytemuck::Pod for TraceFrameData {}

pub fn make_bytes(pos: na::Vector3<f32>, orientation: na::UnitQuaternion<f32>, fov: f32) 
-> Vec<u8> {

    let trace_frame_data = TraceFrameData {
        pos: pos.into(),
        right:
            orientation.transform_vector(&na::Vector3::x_axis()).into(),
        up:
            orientation.transform_vector(&na::Vector3::y_axis()).into(),
        forward:
            orientation.transform_vector(&na::Vector3::z_axis()).into(),
        cotan_half_fov:
            1. / (fov.to_radians() * 0.5).tan(),
        ..Default::default()
    };

    Vec::from(bytemuck::bytes_of(&trace_frame_data))
}