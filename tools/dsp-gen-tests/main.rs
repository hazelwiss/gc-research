#![feature(iter_intersperse)]
#![feature(file_buffered)]
#![feature(iter_array_chunks)]

use std::path::PathBuf;
use tools::dspemit::Emitter;

const SYSTEM_MEM: usize = 24 << 20;
// At least ensure 5MiB of system memory is not taken up by a single test.
const SYSTEM_MEM_LEFT: usize = 5 << 20;
const DISK_MAX_STORAGE: usize = 1400 << 20;

struct Test {
    name: &'static str,
    body: fn(&mut Emitter),
    // If the test is custom and if it is custom, how many of them are there.
    //
    // A custom test is one which does not make use of the generated prologue/epilogue code
    // and completely defines its own input/output code. Although the output code will always
    // remain the same most likely.
    custom: Option<usize>,
}

impl Test {
    const fn new(name: &'static str, body: fn(&mut Emitter)) -> Self {
        Self {
            name,
            body,
            custom: None,
        }
    }
}

const TESTS: &[Test] = &[Test::new("nop", |e| {
    e.nop();
})];

fn main() -> anyhow::Result<()> {
    let out_dir = PathBuf::from(
        std::env::args()
            .nth(1)
            .unwrap_or("../tests/dsp/apps".to_string()),
    );

    if out_dir.exists() {
        std::fs::remove_dir_all(&out_dir)?;
    }
    std::fs::create_dir(&out_dir).expect("failed to create output directory");

    let mut total_bytes_of_tests = 0;
    for cur in TESTS {
        println!("generating {}", cur.name);
        let mut e: Emitter = Emitter::default();

        let total_tasks = u16::try_from(cur.custom.unwrap_or(tools::dsp::GEN_DSP_INPUTS))?;
        e.emit(total_tasks);
        e.emit(cur.custom.is_some() as u16);
        assert!(cur.name.len() < 32);
        let mut name = [0u8; 32];
        for (i, b) in cur.name.as_bytes().iter().cloned().enumerate().take(31) {
            name[i] = b;
        }
        for b in name.iter().cloned().array_chunks::<2>() {
            e.emit(u16::from_be_bytes(b));
        }

        for _ in 0..total_tasks {
            (cur.body)(&mut e);
            e.emit_invalid_mark();
        }

        if e.len() > SYSTEM_MEM - SYSTEM_MEM_LEFT {
            panic!("Too much memory occupied by tests to fit in memory")
        }
        total_bytes_of_tests += e.len();

        let file_bin_name = format!("{}.bin", cur.name);
        let file_bin_path = out_dir.join(&file_bin_name);
        let file_c_path = out_dir.join(format!("dsp-test-{}.c", cur.name));

        println!(
            "creating '{}', '{}'",
            file_bin_path.display(),
            file_c_path.display()
        );

        std::fs::write(
            file_bin_path,
            e.into_inner()
                .iter()
                .flat_map(|&word| word.to_be_bytes())
                .collect::<Vec<u8>>()
                .as_slice(),
        )?;

        std::fs::write(
            file_c_path,
            format!(
                "\
                #include \"../single-task.h\"\n\
                uint8_t task_data[] = {{\n\
                \t#embed \"{file_bin_name}\"\n\
                }};\n\
                #include \"../test-main.h\"\n\
                ",
            ),
        )?;
    }

    if total_bytes_of_tests > DISK_MAX_STORAGE {
        panic!("Cannot fit all tests on disk");
    }

    let files = format!(
        "\
        char* files[] = {{\n{files}\n}};\n\
        int file_cnt = {cnt};\n\
        ",
        files = TESTS
            .iter()
            .map(|t| format!("    \"dvd:/{}.bin\"", t.name))
            .intersperse(",\n".to_string())
            .collect::<String>(),
        cnt = TESTS.len(),
    );

    std::fs::write(
        out_dir.join("dsp-test-all.c"),
        format!(
            "\
            {files}\n\
            #define HAS_DISK
            #include \"../all-tasks.h\"\n\
            #include \"../test-main.h\"\n\
            "
        ),
    )?;

    std::fs::write(
        out_dir.join("dsp-fuzzer.c"),
        format!(
            "\
            {files}\n\
            #include \"../fuzzer-main.h\"\n\
            "
        ),
    )?;

    let mut binary_rules = "".to_string();
    for test in TESTS {
        binary_rules += test.name;
        binary_rules += ".bin ";
    }

    std::fs::write(out_dir.join("build-gc"), {
        let mut contents = "".to_string();
        contents += "include ../../../common/build.mk\n";
        contents += "build: dsp-test-all.iso ";
        for test in TESTS {
            contents += &format!("dsp-test-{}.dol ", test.name);
        }
        contents += "\n";

        for test in TESTS {
            contents += &format!(
                "dsp-test-{name}.dol: dsp-test-{name}.elf\n",
                name = test.name
            );
            contents += &format!("dsp-test-{name}.elf: dsp-test-{name}.o\n", name = test.name);
        }

        contents += &format!("dsp-test-all.iso: dsp-test-all.dol {binary_rules}\n");
        contents += "dsp-test-all.dol: dsp-test-all.elf\n";
        contents += "dsp-test-all.elf: dsp-test-all.o\n";
        contents += "package: build\n";
        contents += "\t@mkdir -p $(PACKAGE_DIR)/dsp\n";
        for test in TESTS {
            contents += &format!(
                "\t@cp dsp-test-{name}.dol $(PACKAGE_DIR)/dsp/dsp-test-{name}.dol\n",
                name = test.name
            );
        }
        contents += "\t@cp dsp-test-all.dol $(PACKAGE_DIR)/dsp/dsp-test-all.dol\n";

        contents
    })?;

    std::fs::write(
        out_dir.join("build-wii"),
        "\
        include ../../../common/build-wii.mk\n\
        build: dsp-fuzzer.dol\n\
        \n\
        dsp-fuzzer.dol: dsp-fuzzer.elf\n\
        dsp-fuzzer.elf: dsp-fuzzer.o\n\
        ",
    )?;

    Ok(())
}
