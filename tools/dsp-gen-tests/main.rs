#![feature(iter_intersperse)]
#![feature(file_buffered)]
#![feature(iter_array_chunks)]

use rand::{Rng, SeedableRng};
use std::path::PathBuf;
use tools::dspemit::{Cond, Emitter, ExtendedOpcode, ExtendedOpcode7, ext};

const SYSTEM_MEM: usize = 24 << 20;
// At least ensure 5MiB of system memory is not taken up by a single test.
const SYSTEM_MEM_LEFT: usize = 5 << 20;
const DISK_MAX_STORAGE: usize = 1400 << 20;

const SEED: [u8; 32] = [
    0xAA, 0x55, 0xCC, 0x33, 0xF0, 0x0F, 0x99, 0x66, 0x5A, 0xA5, 0x3C, 0xC3, 0x96, 0x69, 0x0F, 0xF0,
    0x55, 0xAA, 0x33, 0xCC, 0x0F, 0xF0, 0x66, 0x99, 0xA5, 0x5A, 0xC3, 0x3C, 0x69, 0x96, 0xF0, 0x0F,
];

struct Test {
    name: &'static str,
    body: fn(&mut Emitter, &mut rand::rngs::SmallRng),
    // If the test is custom and if it is custom, how many of them are there.
    //
    // A custom test is one which does not make use of the generated prologue/epilogue code
    // and completely defines its own input/output code. Although the output code will always
    // remain the same most likely.
    custom: Option<usize>,
}

impl Test {
    const fn new(name: &'static str, body: fn(&mut Emitter, &mut rand::rngs::SmallRng)) -> Self {
        Self {
            name,
            body,
            custom: None,
        }
    }
}

fn ext(r: &mut rand::rngs::SmallRng) -> impl ExtendedOpcode {
    match r.random_range(0..8) {
        0..=3 => ext::Nop.extended_opc(),
        4 => ext::Dr(r.random::<u8>()).extended_opc(),
        5 => ext::Ir(r.random::<u8>()).extended_opc(),
        6 => ext::Mv(r.random::<u8>(), r.random::<u8>()).extended_opc(),
        7 => ext::Nr(r.random::<u8>()).extended_opc(),
        _ => unreachable!(),
    }
}

fn ext7(r: &mut rand::rngs::SmallRng) -> impl ExtendedOpcode7 {
    match r.random_range(0..8) {
        0..=3 => ext::Nop.extended_opc(),
        4 => ext::Dr(r.random::<u8>()).extended_opc(),
        5 => ext::Ir(r.random::<u8>()).extended_opc(),
        6 => ext::Mv(r.random::<u8>(), r.random::<u8>()).extended_opc(),
        7 => ext::Nr(r.random::<u8>()).extended_opc(),
        _ => unreachable!(),
    }
}

fn cond(r: &mut rand::rngs::SmallRng) -> Cond {
    match r.random_range(0..16) {
        0 => Cond::Ge,
        1 => Cond::L,
        2 => Cond::G,
        3 => Cond::Le,
        4 => Cond::Nz,
        5 => Cond::Z,
        6 => Cond::Nc,
        7 => Cond::C,
        8 => Cond::x8,
        9 => Cond::x9,
        10 => Cond::xA,
        11 => Cond::xB,
        12 => Cond::Lnz,
        13 => Cond::Lz,
        14 => Cond::O,
        15 => Cond::Always,
        _ => unreachable!(),
    }
}

const TESTS: &[Test] = &[
    // nop test
    Test::new("nop", |e, _| {
        e.nop();
    }),
    // Main opcode tests.
    Test::new("abs", |e, r| {
        e.abs(r.random::<bool>(), ext(r));
    }),
    Test::new("add", |e, r| {
        e.add(r.random::<bool>(), ext(r));
    }),
    Test::new("addarn", |e, r| {
        e.addarn(r.random::<u8>(), r.random::<u8>())
    }),
    Test::new("addax", |e, r| {
        e.addax(r.random::<u8>(), r.random::<u8>(), ext(r))
    }),
    Test::new("addaxl", |e, r| {
        e.addaxl(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("addi", |e, r| {
        e.addi(r.random::<bool>(), r.random());
    }),
    Test::new("addis", |e, r| {
        e.addis(r.random::<bool>(), r.random::<u8>());
    }),
    Test::new("addp", |e, r| e.addp(r.random::<bool>(), ext(r))),
    Test::new("addpaxz", |e, r| {
        e.addpaxz(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("addr", |e, r| {
        e.addr(r.random::<bool>(), r.random::<u8>(), ext(r))
    }),
    Test::new("andc", |e, r| e.andc(r.random::<bool>(), ext(r))),
    Test::new("andcf", |e, r| e.andcf(r.random::<bool>(), r.random())),
    Test::new("andf", |e, r| e.andf(r.random::<bool>(), r.random())),
    Test::new("andi", |e, r| {
        e.andi(r.random::<bool>(), r.random());
    }),
    Test::new("andr", |e, r| {
        e.andr(r.random::<bool>(), r.random::<bool>(), ext7(r))
    }),
    Test::new("asl", |e, r| e.asl(r.random::<bool>(), r.random::<u8>())),
    Test::new("asr", |e, r| {
        e.asr(r.random::<bool>(), r.random::<u8>());
    }),
    Test::new("asrn", |e, _| e.asrn()),
    Test::new("asrnr", |e, r| {
        e.asrnr(r.random::<bool>(), ext7(r));
    }),
    Test::new("asrnrx", |e, r| {
        e.asrnrx(r.random::<bool>(), r.random::<bool>(), ext7(r));
    }),
    Test::new("asr16", |e, r| {
        e.asr16(r.random::<bool>(), ext(r));
    }),
    Test::new("bloop", |e, r| {
        e.bloop(r.random::<u8>(), r.random());
        e.add(r.random::<bool>(), ext(r));
    }),
    Test::new("bloopi", |e, r| {
        e.bloopi(r.random::<u8>(), r.random());
        e.add(r.random::<bool>(), ext(r));
    }),
    Test::new("clr15", |e, r| {
        e.clr15(ext(r));
    }),
    Test::new("clr", |e, r| {
        e.clr(r.random::<bool>(), ext(r));
    }),
    Test::new("clrl", |e, r| {
        e.clrl(r.random::<bool>(), ext(r));
    }),
    Test::new("clrp", |e, r| {
        e.clrp(ext(r));
    }),
    Test::new("cmp", |e, r| {
        e.cmp(ext(r));
    }),
    Test::new("cmpaxh", |e, r| {
        e.cmpaxh(r.random::<bool>(), ext(r));
    }),
    Test::new("cmpi", |e, r| e.cmpi(r.random::<bool>(), r.random())),
    Test::new("dec", |e, r| {
        e.dec(r.random::<bool>(), ext(r));
    }),
    Test::new("decm", |e, r| {
        e.decm(r.random::<bool>(), ext(r));
    }),
    Test::new("iar", |e, r| {
        e.iar(r.random::<u8>());
    }),
    Test::new("ifcc", |e, r| {
        e.ifcc(cond(r));
        e.add(r.random::<bool>(), ext(r));
    }),
    Test::new("inc", |e, r| {
        e.inc(r.random::<bool>(), ext(r));
    }),
    Test::new("incm", |e, r| {
        e.incm(r.random::<bool>(), ext(r));
    }),
    Test::new("loop", |e, r| {
        e.loop_(r.random::<u8>());
        e.add(r.random::<bool>(), ext(r));
    }),
    Test::new("loopi", |e, r| {
        e.loopi(r.random::<u8>());
        e.add(r.random::<bool>(), ext(r))
    }),
    Test::new("lsl", |e, r| {
        e.lsl(r.random::<bool>(), r.random::<u8>());
    }),
    Test::new("lsl16", |e, r| {
        e.lsl16(r.random::<bool>(), ext(r));
    }),
    Test::new("lsr", |e, r| e.lsr(r.random::<bool>(), r.random::<u8>())),
    Test::new("lsrn", |e, _| {
        e.lsrn();
    }),
    Test::new("lsrnr", |e, r| {
        e.lsrnr(r.random::<bool>(), ext7(r));
    }),
    Test::new("lsrnrx", |e, r| {
        e.lsrnrx(r.random::<bool>(), r.random::<bool>(), ext7(r));
    }),
    Test::new("lsr16", |e, r| {
        e.lsr16(r.random::<bool>(), ext(r));
    }),
    Test::new("m0", |e, r| {
        e.m0(ext(r));
    }),
    Test::new("m2", |e, r| {
        e.m2(ext(r));
    }),
    Test::new("madd", |e, r| {
        e.madd(r.random::<bool>(), ext(r));
    }),
    Test::new("maddc", |e, r| {
        e.maddc(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("maddx", |e, r| {
        e.maddx(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("mov", |e, r| e.mov(r.random::<bool>(), ext(r))),
    Test::new("movax", |e, r| {
        e.movax(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("movnp", |e, r| {
        e.movnp(r.random::<bool>(), ext(r));
    }),
    Test::new("movp", |e, r| {
        e.movp(r.random::<bool>(), ext(r));
    }),
    Test::new("movpz", |e, r| {
        e.movpz(r.random::<bool>(), ext(r));
    }),
    Test::new("movr", |e, r| {
        e.movr(r.random::<bool>(), r.random::<u8>(), ext(r));
    }),
    Test::new("mrr", |e, r| e.mrr(r.random::<u8>(), r.random::<u8>())),
    Test::new("msub", |e, r| {
        e.msub(r.random::<bool>(), ext(r));
    }),
    Test::new("msubc", |e, r| {
        e.msubc(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("msubx", |e, r| {
        e.msubc(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("mul", |e, r| {
        e.mul(r.random::<bool>(), ext(r));
    }),
    Test::new("mulac", |e, r| {
        e.mulac(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("mulaxh", |e, r| {
        e.mulaxh(ext(r));
    }),
    Test::new("mulc", |e, r| {
        e.mulc(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("mulcac", |e, r| {
        e.mulcac(
            r.random::<bool>(),
            r.random::<bool>(),
            r.random::<bool>(),
            ext(r),
        );
    }),
    Test::new("mulcmv", |e, r| {
        e.mulcmv(
            r.random::<bool>(),
            r.random::<bool>(),
            r.random::<bool>(),
            ext(r),
        );
    }),
    Test::new("mulcmvz", |e, r| {
        e.mulcmvz(
            r.random::<bool>(),
            r.random::<bool>(),
            r.random::<bool>(),
            ext(r),
        );
    }),
    Test::new("mulmv", |e, r| {
        e.mulmv(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("mulmvz", |e, r| {
        e.mulmvz(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("mulx", |e, r| {
        e.mulx(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("mulxac", |e, r| {
        e.mulxac(
            r.random::<bool>(),
            r.random::<bool>(),
            r.random::<bool>(),
            ext(r),
        );
    }),
    Test::new("mulxmv", |e, r| {
        e.mulxmv(
            r.random::<bool>(),
            r.random::<bool>(),
            r.random::<bool>(),
            ext(r),
        )
    }),
    Test::new("mulxmvz", |e, r| {
        e.mulxmvz(
            r.random::<bool>(),
            r.random::<bool>(),
            r.random::<bool>(),
            ext(r),
        );
    }),
    Test::new("neg", |e, r| e.neg(r.random::<bool>(), ext(r))),
    Test::new("not", |e, r| e.neg(r.random::<bool>(), ext(r))),
    Test::new("orc", |e, r| e.orc(r.random::<bool>(), ext7(r))),
    Test::new("ori", |e, r| {
        e.ori(r.random::<bool>(), r.random());
    }),
    Test::new("orr", |e, r| {
        e.orr(r.random::<bool>(), r.random::<bool>(), ext7(r))
    }),
    Test::new("sbclr", |e, r| {
        e.sbclr(r.random::<u8>());
    }),
    Test::new("sbset", |e, r| {
        e.sbset(r.random::<u8>());
    }),
    Test::new("set15", |e, r| {
        e.set15(ext(r));
    }),
    Test::new("set16", |e, r| {
        e.set16(ext(r));
    }),
    Test::new("set40", |e, r| {
        e.set40(ext(r));
    }),
    Test::new("sub", |e, r| {
        e.sub(r.random::<bool>(), ext(r));
    }),
    Test::new("subarn", |e, r| {
        e.subarn(r.random::<u8>());
    }),
    Test::new("subax", |e, r| {
        e.subax(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("subp", |e, r| {
        e.subp(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("subr", |e, r| {
        e.subr(r.random::<bool>(), r.random::<u8>(), ext(r))
    }),
    Test::new("tst", |e, r| {
        e.tst(r.random::<bool>(), ext(r));
    }),
    Test::new("tstaxh", |e, r| {
        e.tstaxh(r.random::<bool>(), ext(r));
    }),
    Test::new("tstprod", |e, r| {
        e.tstprod(ext(r));
    }),
    Test::new("xorc", |e, r| {
        e.xorc(r.random::<bool>(), ext7(r));
    }),
    Test::new("xori", |e, r| {
        e.xori(r.random::<bool>(), r.random());
    }),
    Test::new("xorr", |e, r| {
        e.xorr(r.random::<bool>(), r.random::<bool>(), ext7(r));
    }),
    // Extended opcode tests.
    Test::new("dr", |e, r| e.nx(ext::Dr(r.random::<u8>()))),
    Test::new("ir", |e, r| e.nx(ext::Ir(r.random::<u8>()))),
    Test::new("mv", |e, r| {
        e.nx(ext::Mv(r.random::<u8>(), r.random::<u8>()))
    }),
    Test::new("nr", |e, r| e.nx(ext::Nr(r.random::<u8>()))),
    // TODO:
    // - load/store main operations
    // - load/store extended operations
    // - branching (jmp, call, ret, rti)
    // - nested loop tests
    // - results from extended operations coliding with main operation
    // - exception specific tests
];

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

    let mut rand = rand::rngs::SmallRng::from_seed(SEED);

    let result_dir = PathBuf::from("../tests/dsp/results");

    let mut binary_rules = "".to_string();
    let mut results_files = vec![];
    let mut total_bytes_of_tests = 0;
    for cur in TESTS {
        binary_rules += cur.name;
        binary_rules += ".bin ";

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
            (cur.body)(&mut e, &mut rand);
            e.emit_invalid_mark();
        }

        if e.len() > SYSTEM_MEM - SYSTEM_MEM_LEFT {
            panic!("Too much memory occupied by tests to fit in memory")
        }
        total_bytes_of_tests += e.len();

        let file_bin_name = format!("{}.bin", cur.name);
        let file_res_name = format!("{}.result.bin", cur.name);
        let file_bin_path = out_dir.join(&file_bin_name);
        let file_res_path = result_dir.join(&file_res_name);
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

        let results_find = if file_res_path.exists() {
            let result_file = format!("../results/{}.result.bin", cur.name);
            binary_rules += result_file.as_str();
            binary_rules += " ";

            results_files.push(Some(result_file));

            format!(
                "uint8_t result_data_[] = {{\n\t#embed \"../results/{}\"\n}};\nuint8_t *result_data = result_data_;",
                file_res_name
            )
        } else {
            results_files.push(None);

            "uint8_t *result_data = 0;".to_string()
        };
        std::fs::write(
            file_c_path,
            format!(
                "\
                #include \"../single-task.h\"\n\
                uint8_t task_data[] = {{\n\
                \t#embed \"{file_bin_name}\"\n\
                }};\n\
                {results_find}\n\
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
        char* result_files[] = {{\n{res}\n}};\n\
        int file_cnt = {cnt};\n\
        ",
        files = TESTS
            .iter()
            .map(|t| format!("    \"dvd:/{}.bin\"", t.name))
            .intersperse(",\n".to_string())
            .collect::<String>(),
        res = results_files
            .iter()
            .map(|s| s
                .as_ref()
                .map(|s| format!("\t\"dvd:/{}.bin\"", s))
                .unwrap_or_else(|| "\t0".to_string()))
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
