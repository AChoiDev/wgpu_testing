fn main() {
    println!("cargo:rerun-if-changed=src/shaders");

    std::fs::create_dir_all("src/spirv").unwrap();

    let mut compiler = shaderc::Compiler::new().unwrap();

    std::fs::read_dir("src/shaders").unwrap()
    .map(|entry| entry.unwrap())
    .filter(|entry| entry.file_type().unwrap().is_file())
    .map(|entry| entry.path())
    .map(|in_path| {
        (
            in_path.extension().and_then(|ext| {
                match ext.to_string_lossy().as_ref() {
                    "vert" => Some(shaderc::ShaderKind::Vertex),
                    "frag" => Some(shaderc::ShaderKind::Fragment),
                    "comp" => Some(shaderc::ShaderKind::Compute),
                    _ => None,
                } 
            }),
            in_path,
        )
    })
    .filter_map(|(shader_type, in_path)| 
        shader_type.and_then(|st| Some((st, in_path)))
    )
    .for_each(|(shader_type, in_path)|
        std::fs::write(
            format!("src/spirv/{}.spv", in_path.file_name().unwrap().to_string_lossy()),
            compiler.compile_into_spirv(
                std::fs::read_to_string(&in_path).unwrap().as_str(),
                shader_type,
                in_path.file_name().unwrap().to_str().unwrap(),
                "main",
                None
            ).unwrap().as_binary_u8()
            //glsl_to_spirv::compile(std::fs::read_to_string(&in_path).unwrap(), shader_type)
            //.unwrap()
        ).unwrap()
    );
}