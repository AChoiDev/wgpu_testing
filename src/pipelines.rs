use crate::bind_group_layouts::BindGroupLayouts;

pub struct Pipelines {
    pub process: wgpu::RenderPipeline,
    pub trace: wgpu::ComputePipeline,
}


impl Pipelines {
    pub fn new(device: &wgpu::Device, bind_group_layouts : &BindGroupLayouts, 
            sc_desc: &wgpu::SwapChainDescriptor)
    -> Self {

        let modules = crate::shader_modules::ShaderModules::new(&device);

    let compute_pipeline_layout = 
        device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts:
                    &[
                        &bind_group_layouts.trace,
                    ],
                push_constant_ranges: &[],
            }
        );

    let compute_pipeline = device.create_compute_pipeline(
        &wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&compute_pipeline_layout),
            compute_stage: wgpu::ProgrammableStageDescriptor {
                module: &modules.gradient_comp,
                entry_point: "main",
            },
        }
    );
    let process_pipeline_layout = 
        device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts:
                    &[
                        &bind_group_layouts.process,
                    ],
                push_constant_ranges: &[],
            }
        );

   
    let process_pipeline = device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&process_pipeline_layout),
            vertex_stage: 
                wgpu::ProgrammableStageDescriptor {
                    module: &modules.screen_vert,
                    entry_point: "main"
                },
            fragment_stage:
                Some(
                    wgpu::ProgrammableStageDescriptor {
                        module: &modules.process_frag,
                        entry_point: "main"
                    }
                ),
            rasterization_state: None,
            primitive_topology: wgpu::PrimitiveTopology::TriangleStrip,
            color_states: 
                &[sc_desc.format.into()],
            depth_stencil_state: None,
            vertex_state:
                wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint32,
                    vertex_buffers:
                        &[
                            wgpu::VertexBufferDescriptor {
                                stride: 8,
                                step_mode: wgpu::InputStepMode::Vertex,
                                attributes: &wgpu::vertex_attr_array![0 => Float2],
                            }
                        ],
                },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        },
    );

        Self {
            trace: compute_pipeline,
            process: process_pipeline,
        }
    }
}