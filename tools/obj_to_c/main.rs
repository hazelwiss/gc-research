use std::path::PathBuf;

/// obj to C
#[derive(argp::FromArgs)]
struct Args {
    #[argp(positional)]
    output_h: PathBuf,
    #[argp(positional)]
    output_c: PathBuf,
    #[argp(positional, greedy)]
    input: Vec<PathBuf>,
}

fn main() {
    let args: Args = argp::parse_args_or_exit(&argp::HelpStyle::default());

    let mut models = vec![];

    for input in &args.input {
        let (loaded, _) = tobj::load_obj(
            input,
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ignore_points: true,
                ignore_lines: true,
            },
        )
        .unwrap_or_else(|e| panic!("{e}: failed to load object '{}'", input.display()));
        models.extend(loaded);
    }

    let mut imports = String::new();
    let mut defs = String::new();

    for (i, model) in models.into_iter().enumerate() {
        let mesh = &model.mesh;

        println!("parsing {}", model.name);
        println!("vertices {}", mesh.indices.len());
        println!("positions {}", mesh.positions.len());
        println!("normals {}", mesh.normals.len());
        println!("colour {}", mesh.vertex_color.len());
        println!("texcoords {}", mesh.texcoords.len());

        let name = if model
            .name
            .chars()
            .next()
            .as_ref()
            .is_some_and(char::is_ascii)
        {
            model.name
        } else {
            format!("_{i}")
        };
        imports.push_str(&format!("extern struct ModelVertex {name}[];\n",));
        imports.push_str(&format!("extern unsigned long {name}_len;\n",));

        defs.push_str(&format!("struct ModelVertex {name}[] = {{\n"));
        let mut vtx_len = 0;
        for &i in mesh.indices.iter() {
            let vtx = (i * 3) as usize;
            defs.push_str("    (struct ModelVertex) {\n");
            if !mesh.positions.is_empty() {
                let x = mesh.positions[vtx];
                let y = mesh.positions[vtx + 1];
                let z = mesh.positions[vtx + 2];
                defs.push_str(&format!("        .pos = {{{x},{y},{z}}},\n"));
            } else {
                defs.push_str("        .pos = {0,0,0},\n");
            }
            if !mesh.normals.is_empty() {
                let x = mesh.normals[vtx];
                let y = mesh.normals[vtx + 1];
                let z = mesh.normals[vtx + 2];
                defs.push_str(&format!("        .nrm = {{{x},{y},{z}}},\n"));
            } else {
                defs.push_str("        .nrm = {0,0,0},\n");
            }
            if !mesh.vertex_color.is_empty() {
                let x = mesh.vertex_color[vtx];
                let y = mesh.vertex_color[vtx + 1];
                let z = mesh.vertex_color[vtx + 2];
                defs.push_str(&format!("        .col = {{{x},{y},{z},1.0}},\n"));
            } else {
                defs.push_str("        .col = {0,0,0,1},\n");
            }
            if !mesh.texcoords.is_empty() {
                let vtx = (i * 2) as usize;
                let u = mesh.texcoords[vtx];
                let v = mesh.texcoords[vtx + 1];
                defs.push_str(&format!("        .tex = {{{u},{v}}},\n"));
            } else {
                defs.push_str("        .tex = {0,0},\n");
            }
            defs.push_str("    },\n");
            vtx_len += 1;
        }
        defs.push_str("};\n");
        defs.push_str(&format!("unsigned long {name}_len = {vtx_len};\n"));
    }

    let out_header = format!(
        "\
struct ModelVertex {{
    struct {{ float x, y, z; }} pos, nrm;
    struct {{ float r, g, b, a; }} col;
    struct {{ float u, v; }} tex;
}};

{imports}"
    );
    let out_header_file = args
        .output_h
        .file_name()
        .expect("missing final name in path")
        .display();
    let out_source = format!(
        "\
        #include \"{out_header_file}\"\n\
        \n\
        {defs}\
        "
    );

    std::fs::write(&args.output_h, out_header).unwrap_or_else(|e| {
        panic!(
            "{e}: failed to write to output file '{}'",
            args.output_h.display()
        )
    });
    std::fs::write(&args.output_c, out_source).unwrap_or_else(|e| {
        panic!(
            "{e}: failed to write to output file '{}'",
            args.output_h.display()
        )
    });
}
