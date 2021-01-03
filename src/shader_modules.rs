pub struct ShaderModules {
    pub screen_vert: wgpu::ShaderModule,
    pub process_frag: wgpu::ShaderModule,
    pub gradient_comp: wgpu::ShaderModule,
}

impl ShaderModules {
    pub fn new(device: &wgpu::Device)
    -> Self {
        Self {
            gradient_comp: 
                device.create_shader_module(
                    wgpu::include_spirv!("spirv/gradient.comp.spv")
                ),
            screen_vert: 
                device.create_shader_module(
                    wgpu::include_spirv!("spirv/screen.vert.spv")
                ),
            process_frag: 
                device.create_shader_module(
                    wgpu::include_spirv!("spirv/process.frag.spv")
                ),
        }
    }
}