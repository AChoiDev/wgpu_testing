
fn main() {
    println!("cargo:rerun-if-changed=src/shaders/*");
    std::fs::create_dir_all("src/spirv").unwrap();

    let write_inputs = make_write_inputs_root_files();

    write_inputs.into_iter()
    .chain(make_write_inputs_sum_shader().into_iter())
    .for_each(|(data, path)|{
        println!("{}", path.clone());
        std::fs::write(
            path,
            data
        ).unwrap()}
    );
}

enum Axis {
    X, Y, Z
}
fn swizzle_option<'a>(axis: &Axis) 
-> shaderc::CompileOptions<'a> {
    let mut option = shaderc::CompileOptions::new().unwrap();

    let base = match axis {
        Axis::X => "{}, invoc.x, invoc.y",
        Axis::Y => "invoc.x, {}, invoc.y",
        Axis::Z => "invoc.x, invoc.y, {}",
    };

    shaderc::CompileOptions::add_macro_definition(&mut option, 
        "SWIZ_INIT", Some(base.replace("{}", "1").as_str()));
    shaderc::CompileOptions::add_macro_definition(&mut option, 
        "SWIZ_INDEX", Some(base.replace("{}", "i").as_str()));

    option
}

pub fn make_write_inputs_sum_shader()
-> Vec<(Vec<u8>, String)> {
    let mut compiler = shaderc::Compiler::new().unwrap();

    [Axis::X, Axis::Y, Axis::Z]
    .iter()
    .map(|axis| swizzle_option(axis))
    .enumerate()
    .map(|(i, option)| 
        (
            compiler.compile_into_spirv(
                std::fs::read_to_string("src/shaders/sum/sum.comp").unwrap().as_str(),
                shaderc::ShaderKind::Compute, 
                "sum.comp", 
                "main", 
                Some(&option)
            ).unwrap().as_binary_u8().to_vec(),
            "src/spirv/sum_{}.comp.spv".replace("{}", i.to_string().as_str()),
        )
    ).collect()
}


pub fn make_write_inputs_root_files()
-> Vec<(Vec<u8>, String)> {
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
    .map(|(shader_type, in_path)|
        (
            compiler.compile_into_spirv(
                std::fs::read_to_string(&in_path).unwrap().as_str(),
                shader_type,
                in_path.file_name().unwrap().to_str().unwrap(),
                "main",
                None
            ).unwrap().as_binary_u8().to_vec(),
            format!("src/spirv/{}.spv", in_path.file_name().unwrap().to_string_lossy()),
        )
    ).collect()
}