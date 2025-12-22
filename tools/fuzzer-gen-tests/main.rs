#![feature(iter_intersperse)]
#![feature(file_buffered)]

use std::path::PathBuf;
use tools::dspemit::{Cond, Emitter};

const MAX_LEN: usize = 0x1000;
const DEFAULT_CNT: usize = 20000;

const SYSTEM_MEM: usize = 24 << 20;
// At least ensure 5MiB of system memory is not taken up by a single test.
const SYSTEM_MEM_LEFT: usize = 5 << 20;
const DISK_MAX_STORAGE: usize = 1400 << 20;

struct InputState {
    regs: [u16; 32],
}

struct Tests {
    name: &'static str,
    instr: fn(&mut Emitter),
    iters: usize,
}

const TESTS: &[Tests] = &[Tests {
    name: "nop",
    instr: |e| {
        e.nop();
    },
    iters: DEFAULT_CNT,
}];

fn gen_input_state() -> InputState {
    InputState {
        regs: std::array::from_fn(|_| rand::random()),
    }
}

fn start(e: &mut Emitter) {
    for _ in 0..16 {
        e.nop();
    }
}

fn end(e: &mut Emitter) {
    // Jump back to IROM start
    e.jcc(Cond::Always, 0x8000);
}

/// Insert test prelude
fn prelog(e: &mut Emitter, input: Option<InputState>) {
    let input = input.unwrap_or(gen_input_state());
    for (i, &input) in input.regs.iter().enumerate() {
        e.lri(i as u8, input);
    }
}

/// Insert test epilogue
fn epilogue(e: &mut Emitter) {
    for i in 0..32 {
        e.si(0xfc, 0);
        e.sr(i as u8, 0xfffd);
        e.lr(31, 0xfffc);
        e.andf(true, 1 << 15);
        e.jcc(Cond::Lnz, (u16::try_from(e.len()).unwrap()).wrapping_sub(4));
    }
}

fn main() {
    let out_dir = PathBuf::from(
        std::env::args()
            .nth(1)
            .unwrap_or("../tests/dsp/.gen".to_string()),
    );

    if out_dir.exists() {
        println!(
            "'{}' already exists, will not regenerate dsp files",
            out_dir.display()
        );
        return;
    }
    std::fs::create_dir(&out_dir).expect("failed to create output directory");

    // Reserve the initial 18 bytes for the header.
    let mut out: Vec<u16> = vec![];
    let mut e: Emitter = Emitter::default();

    let flush = |out: &mut Vec<u16>, e: &mut Emitter, p: usize, chunk_tasks: u16| {
        let mut block = e.drain_into_emitter(p);
        end(&mut block);
        if block.len() > MAX_LEN {
            panic!("bug")
        }
        while block.len() < MAX_LEN {
            block.nop();
        }
        // Push the chunk header
        out.push(u16::try_from(block.len()).unwrap().to_be());
        out.push(chunk_tasks.to_be());
        // Push the chunk data
        out.extend(&block);
        block.clear();

        start(&mut block);
        block.append(e);
        *e = block;
    };

    let mut total_bytes_of_tests = 0;
    for cur in TESTS {
        println!("generating {}", cur.name);

        start(&mut e);
        let mut chunk_tasks = 0u16;
        let mut total_tasks = 0u16;
        let mut trailing = false;
        for _ in 0..cur.iters {
            trailing = true;

            let p = e.len();
            prelog(&mut e, None);
            (cur.instr)(&mut e);
            epilogue(&mut e);

            let total_len = e.len();
            if total_len >= MAX_LEN {
                flush(&mut out, &mut e, p, core::mem::take(&mut chunk_tasks));
                trailing = false;
            }

            chunk_tasks += 1;
            total_tasks += 1;
        }
        if trailing {
            let l = e.len();
            flush(&mut out, &mut e, l, chunk_tasks);
        }

        if out.len() > SYSTEM_MEM - SYSTEM_MEM_LEFT {
            panic!("Too much memory occupied by tests to fit in memory")
        }
        total_bytes_of_tests += out.len();

        assert!(cur.name.len() < 31);
        let mut name_bytes = [0; 32];
        for (i, &b) in cur.name.as_bytes().iter().enumerate() {
            name_bytes[i] = b;
        }

        let file_bin_path = out_dir.join(format!("{}.bin", cur.name));
        let file_c_path = out_dir.join(format!("{}.c", cur.name));
        println!(
            "creating '{}', '{}'",
            file_bin_path.display(),
            file_c_path.display()
        );

        let bytes_iter = name_bytes
            .iter()
            .cloned()
            .chain(total_tasks.to_be_bytes())
            .chain(u32::try_from(out.len()).unwrap().to_be_bytes())
            .chain(out.iter().flat_map(|&word| word.to_be_bytes()));

        std::fs::write(
            file_bin_path,
            bytes_iter.clone().collect::<Vec<u8>>().as_slice(),
        )
        .expect("failed to write binary output file");
        std::fs::write(
            file_c_path,
            format!(
                "\
                #include <stdint.h>\n\
                #include \"../tasks.h\"\n\
                static uint8_t data__[] = {{\n\
                \
                {data}\n\
                }};\n\
                struct task* tasks_advance(void) {{\n\
                    static int gotten = 0;\n\
                    if (!gotten) {{\n\
                        gotten = 1;\n\
                        return (struct task*)&data__;\n\
                    }}\n\
                    return 0;\n\
                }}\n\
                uint64_t tasks_len(void) {{ return 1; }}\n\
                #include \"../test-main.h\"\n\
                ",
                data = bytes_iter
                    .map(|v| format!("    {v:#x}"))
                    .intersperse(",\n".to_string())
                    .collect::<String>()
            ),
        )
        .expect("failed to write binary output file");
    }

    if total_bytes_of_tests > DISK_MAX_STORAGE {
        panic!("Cannot fit all tests on disk");
    }

    std::fs::write(out_dir.join("build-gc"), {
        let mut contents = "".to_string();
        contents += "build: dsp-test-all ";
        for test in TESTS {
            contents += &format!("dsp-test-{}.dol ", test.name);
        }
        contents += "\n";

        for test in TESTS {
            contents += &format!(
                "dsp-test-{name}.dol: dsp-test-{name}.elf\n",
                name = test.name
            );
            contents += &format!("dsp-test-{name}.elf: {name}.o\n", name = test.name);
        }

        contents += "dsp-test-all.dol: dsp-test-all.elf\n";
        contents += "dsp-test-all.elf: all.o\n";

        contents
    })
    .expect("failed to create gamecube makefile");

    std::fs::write(out_dir.join("build-wii"), {
        "\
        build: dsp-fuzzer.dol\n\
        \
        dsp-fuzzer.dol: dsp-fuzzer.elf\n\
        dsp-fuzzer.elf: fuzzer.o\n\
        "
    })
    .expect("failed to create gamecube makefile");
}
