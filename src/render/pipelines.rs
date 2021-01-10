use super::bind_group_layouts::BindGroupLayouts;

pub struct Pipelines {
    pub process: wgpu::RenderPipeline,
    pub march: wgpu::ComputePipeline,
    pub cone_march: wgpu::ComputePipeline,
    pub fill_sum_table: wgpu::ComputePipeline,
    pub depth_shade: wgpu::ComputePipeline,
    pub sum_table_passes: Vec<wgpu::ComputePipeline>,
    pub fill_bit_volume: wgpu::ComputePipeline,
    pub halve_bit_volume: wgpu::ComputePipeline,
}

impl Pipelines {
    pub fn new(device: &wgpu::Device, bind_group_layouts : &BindGroupLayouts, 
            sc_desc: &wgpu::SwapChainDescriptor)
    -> Self {
        Self {
            cone_march:
                make_compute_pipeline(
                    wgpu::include_spirv!("../spirv/cone_march.comp.spv"),
                    &device, 
                    &[
                        &bind_group_layouts.cone_march,
                        &bind_group_layouts.view,
                    ]
                ),
            march:
                make_compute_pipeline(
                    wgpu::include_spirv!("../spirv/primary_march.comp.spv"),
                    &device,
                    &[
                        &bind_group_layouts.map,
                        &bind_group_layouts.view,
                        &bind_group_layouts.march,
                    ]
                ),
            process:
                make_process_pipeline(&device, 
                    &sc_desc, 
                    &[
                        &bind_group_layouts.process
                    ]
                ),
            fill_sum_table:
                make_compute_pipeline(
                    wgpu::include_spirv!("../spirv/fill_sum_table.comp.spv"),
                    &device,
                    &[
                        &bind_group_layouts.map,
                        &bind_group_layouts.edit_sum_table,
                    ]
                ),
            depth_shade: 
                make_compute_pipeline(
                    wgpu::include_spirv!("../spirv/depth_shade.comp.spv"), 
                    &device, 
                    &[
                        &bind_group_layouts.map,
                        &bind_group_layouts.view,
                        &bind_group_layouts.depth_shade,
                    ]
                ),
            sum_table_passes: make_sum_table_passes(&device, &bind_group_layouts),
            fill_bit_volume:
                make_compute_pipeline(
                    wgpu::include_spirv!("../spirv/fill_bit_volume.comp.spv"),
                    &device,
                    &[
                        &bind_group_layouts.map,
                        &bind_group_layouts.edit_map,
                    ]
                ),
            halve_bit_volume:
                make_compute_pipeline(
                    wgpu::include_spirv!("../spirv/halve_bit_volume.comp.spv"),
                    &device, 
                    &[
                        &bind_group_layouts.halve_map,
                    ]
                ),
            
        }
    }
}

fn make_sum_table_passes(device: &wgpu::Device, bind_group_layouts: &BindGroupLayouts) 
-> Vec<wgpu::ComputePipeline> {
    vec![
        wgpu::include_spirv!("../spirv/sum_0.comp.spv"),
        wgpu::include_spirv!("../spirv/sum_1.comp.spv"),
        wgpu::include_spirv!("../spirv/sum_2.comp.spv"),
    ]
    .into_iter()
    .map(|source|
        make_compute_pipeline(
            source,
            &device,
            &[
                &bind_group_layouts.map,
                &bind_group_layouts.edit_sum_table,
            ]
        ),
    ).collect()
}


fn make_compute_pipeline(source: wgpu::ShaderModuleSource, device: &wgpu::Device, bind_group_layouts: &[&wgpu::BindGroupLayout])
-> wgpu::ComputePipeline {

    let layout = 
        device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts,
                push_constant_ranges: &[],
            }
        );

    device.create_compute_pipeline(
        &wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&layout),
            compute_stage: wgpu::ProgrammableStageDescriptor {
                module: 
                    &device.create_shader_module(
                        source,
                    ),
                entry_point: "main",
            },
        }
    )
}

fn make_process_pipeline(device: &wgpu::Device, 
    sc_desc: &wgpu::SwapChainDescriptor,
    bind_group_layouts: &[&wgpu::BindGroupLayout]) 
-> wgpu::RenderPipeline {
    let layout = 
        device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts,
                push_constant_ranges: &[],
            }
        );

    device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex_stage: 
                wgpu::ProgrammableStageDescriptor {
                    module:
                        &device.create_shader_module(
                            wgpu::include_spirv!("../spirv/screen.vert.spv")
                        ),
                    entry_point: "main"
                },
            fragment_stage:
                Some(
                    wgpu::ProgrammableStageDescriptor {
                        module:
                            &device.create_shader_module(
                                wgpu::include_spirv!("../spirv/process.frag.spv")
                            ),
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
    )
}