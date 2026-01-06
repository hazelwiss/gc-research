#![feature(iter_intersperse)]
#![feature(file_buffered)]
#![feature(iter_array_chunks)]

use rand::{Rng, SeedableRng};
use std::path::PathBuf;
use tools::{
    dsp::InputState,
    dspemit::{Cond, Emitter, ExtendedOpcode, ExtendedOpcode7, ext, regs},
};

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
    body: fn(&mut Emitter, &mut rand::rngs::SmallRng, u16),
}

impl Test {
    const fn new(
        name: &'static str,
        body: fn(&mut Emitter, &mut rand::rngs::SmallRng, u16),
    ) -> Self {
        Self { name, body }
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
    Test::new("nop", |e, _, _| {
        e.nop();
    }),
    // Main opcode tests.
    Test::new("abs", |e, r, _| {
        e.abs(r.random::<bool>(), ext(r));
    }),
    Test::new("add", |e, r, _| {
        e.add(r.random::<bool>(), ext(r));
    }),
    Test::new("addarn", |e, r, _| {
        e.addarn(r.random::<u8>(), r.random::<u8>())
    }),
    Test::new("addax", |e, r, _| {
        e.addax(r.random::<u8>(), r.random::<u8>(), ext(r))
    }),
    Test::new("addaxl", |e, r, _| {
        e.addaxl(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("addi", |e, r, _| {
        e.addi(r.random::<bool>(), r.random());
    }),
    Test::new("addis", |e, r, _| {
        e.addis(r.random::<bool>(), r.random::<u8>());
    }),
    Test::new("addp", |e, r, _| e.addp(r.random::<bool>(), ext(r))),
    Test::new("addpaxz", |e, r, _| {
        e.addpaxz(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("addr", |e, r, _| {
        e.addr(r.random::<bool>(), r.random::<u8>(), ext(r))
    }),
    Test::new("andc", |e, r, _| e.andc(r.random::<bool>(), ext(r))),
    Test::new("andcf", |e, r, _| e.andcf(r.random::<bool>(), r.random())),
    Test::new("andf", |e, r, _| e.andf(r.random::<bool>(), r.random())),
    Test::new("andi", |e, r, _| {
        e.andi(r.random::<bool>(), r.random());
    }),
    Test::new("andr", |e, r, _| {
        e.andr(r.random::<bool>(), r.random::<bool>(), ext7(r))
    }),
    Test::new("asl", |e, r, _| e.asl(r.random::<bool>(), r.random::<u8>())),
    Test::new("asr", |e, r, _| {
        e.asr(r.random::<bool>(), r.random::<u8>());
    }),
    Test::new("asrn", |e, _, _| e.asrn()),
    Test::new("asrnr", |e, r, _| {
        e.asrnr(r.random::<bool>(), ext7(r));
    }),
    Test::new("asrnrx", |e, r, _| {
        e.asrnrx(r.random::<bool>(), r.random::<bool>(), ext7(r));
    }),
    Test::new("asr16", |e, r, _| {
        e.asr16(r.random::<bool>(), ext(r));
    }),
    Test::new("clr15", |e, r, _| {
        e.clr15(ext(r));
    }),
    Test::new("clr", |e, r, _| {
        e.clr(r.random::<bool>(), ext(r));
    }),
    Test::new("clrl", |e, r, _| {
        e.clrl(r.random::<bool>(), ext(r));
    }),
    Test::new("clrp", |e, r, _| {
        e.clrp(ext(r));
    }),
    Test::new("cmp", |e, r, _| {
        e.cmp(ext(r));
    }),
    Test::new("cmpaxh", |e, r, _| {
        e.cmpaxh(r.random::<bool>(), ext(r));
    }),
    Test::new("cmpi", |e, r, _| e.cmpi(r.random::<bool>(), r.random())),
    Test::new("dec", |e, r, _| {
        e.dec(r.random::<bool>(), ext(r));
    }),
    Test::new("decm", |e, r, _| {
        e.decm(r.random::<bool>(), ext(r));
    }),
    Test::new("iar", |e, r, _| {
        e.iar(r.random::<u8>());
    }),
    Test::new("ifcc", |e, r, _| {
        e.ifcc(cond(r));
        if r.random::<bool>() {
            e.add(r.random::<bool>(), ext(r));
        } else {
            e.addi(r.random::<bool>(), r.random());
        }
    }),
    Test::new("inc", |e, r, _| {
        e.inc(r.random::<bool>(), ext(r));
    }),
    Test::new("incm", |e, r, _| {
        e.incm(r.random::<bool>(), ext(r));
    }),
    Test::new("loop", |e, r, _| {
        e.loop_(r.random::<u8>());
        e.add(r.random::<bool>(), ext(r));
    }),
    Test::new("loopi", |e, r, _| {
        e.loopi(r.random::<u8>());
        e.add(r.random::<bool>(), ext(r))
    }),
    Test::new("lsl", |e, r, _| {
        e.lsl(r.random::<bool>(), r.random::<u8>());
    }),
    Test::new("lsl16", |e, r, _| {
        e.lsl16(r.random::<bool>(), ext(r));
    }),
    Test::new("lsr", |e, r, _| e.lsr(r.random::<bool>(), r.random::<u8>())),
    Test::new("lsrn", |e, _, _| {
        e.lsrn();
    }),
    Test::new("lsrnr", |e, r, _| {
        e.lsrnr(r.random::<bool>(), ext7(r));
    }),
    Test::new("lsrnrx", |e, r, _| {
        e.lsrnrx(r.random::<bool>(), r.random::<bool>(), ext7(r));
    }),
    Test::new("lsr16", |e, r, _| {
        e.lsr16(r.random::<bool>(), ext(r));
    }),
    Test::new("m0", |e, r, _| {
        e.m0(ext(r));
    }),
    Test::new("m2", |e, r, _| {
        e.m2(ext(r));
    }),
    Test::new("madd", |e, r, _| {
        e.madd(r.random::<bool>(), ext(r));
    }),
    Test::new("maddc", |e, r, _| {
        e.maddc(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("maddx", |e, r, _| {
        e.maddx(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("mov", |e, r, _| e.mov(r.random::<bool>(), ext(r))),
    Test::new("movax", |e, r, _| {
        e.movax(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("movnp", |e, r, _| {
        e.movnp(r.random::<bool>(), ext(r));
    }),
    Test::new("movp", |e, r, _| {
        e.movp(r.random::<bool>(), ext(r));
    }),
    Test::new("movpz", |e, r, _| {
        e.movpz(r.random::<bool>(), ext(r));
    }),
    Test::new("movr", |e, r, _| {
        e.movr(r.random::<bool>(), r.random::<u8>(), ext(r));
    }),
    Test::new("mrr", |e, r, _| e.mrr(r.random::<u8>(), r.random::<u8>())),
    Test::new("msub", |e, r, _| {
        e.msub(r.random::<bool>(), ext(r));
    }),
    Test::new("msubc", |e, r, _| {
        e.msubc(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("msubx", |e, r, _| {
        e.msubc(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("mul", |e, r, _| {
        e.mul(r.random::<bool>(), ext(r));
    }),
    Test::new("mulac", |e, r, _| {
        e.mulac(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("mulaxh", |e, r, _| {
        e.mulaxh(ext(r));
    }),
    Test::new("mulc", |e, r, _| {
        e.mulc(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("mulcac", |e, r, _| {
        e.mulcac(
            r.random::<bool>(),
            r.random::<bool>(),
            r.random::<bool>(),
            ext(r),
        );
    }),
    Test::new("mulcmv", |e, r, _| {
        e.mulcmv(
            r.random::<bool>(),
            r.random::<bool>(),
            r.random::<bool>(),
            ext(r),
        );
    }),
    Test::new("mulcmvz", |e, r, _| {
        e.mulcmvz(
            r.random::<bool>(),
            r.random::<bool>(),
            r.random::<bool>(),
            ext(r),
        );
    }),
    Test::new("mulmv", |e, r, _| {
        e.mulmv(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("mulmvz", |e, r, _| {
        e.mulmvz(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("mulx", |e, r, _| {
        e.mulx(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("mulxac", |e, r, _| {
        e.mulxac(
            r.random::<bool>(),
            r.random::<bool>(),
            r.random::<bool>(),
            ext(r),
        );
    }),
    Test::new("mulxmv", |e, r, _| {
        e.mulxmv(
            r.random::<bool>(),
            r.random::<bool>(),
            r.random::<bool>(),
            ext(r),
        )
    }),
    Test::new("mulxmvz", |e, r, _| {
        e.mulxmvz(
            r.random::<bool>(),
            r.random::<bool>(),
            r.random::<bool>(),
            ext(r),
        );
    }),
    Test::new("neg", |e, r, _| e.neg(r.random::<bool>(), ext(r))),
    Test::new("not", |e, r, _| e.neg(r.random::<bool>(), ext(r))),
    Test::new("orc", |e, r, _| e.orc(r.random::<bool>(), ext7(r))),
    Test::new("ori", |e, r, _| {
        e.ori(r.random::<bool>(), r.random());
    }),
    Test::new("orr", |e, r, _| {
        e.orr(r.random::<bool>(), r.random::<bool>(), ext7(r))
    }),
    Test::new("sbclr", |e, r, _| {
        e.sbclr(r.random::<u8>());
    }),
    Test::new("sbset", |e, r, _| {
        e.sbset(r.random::<u8>());
    }),
    Test::new("set15", |e, r, _| {
        e.set15(ext(r));
    }),
    Test::new("set16", |e, r, _| {
        e.set16(ext(r));
    }),
    Test::new("set40", |e, r, _| {
        e.set40(ext(r));
    }),
    Test::new("sub", |e, r, _| {
        e.sub(r.random::<bool>(), ext(r));
    }),
    Test::new("subarn", |e, r, _| {
        e.subarn(r.random::<u8>());
    }),
    Test::new("subax", |e, r, _| {
        e.subax(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("subp", |e, r, _| {
        e.subp(r.random::<bool>(), r.random::<bool>(), ext(r));
    }),
    Test::new("subr", |e, r, _| {
        e.subr(r.random::<bool>(), r.random::<u8>(), ext(r))
    }),
    Test::new("tst", |e, r, _| {
        e.tst(r.random::<bool>(), ext(r));
    }),
    Test::new("tstaxh", |e, r, _| {
        e.tstaxh(r.random::<bool>(), ext(r));
    }),
    Test::new("tstprod", |e, r, _| {
        e.tstprod(ext(r));
    }),
    Test::new("xorc", |e, r, _| {
        e.xorc(r.random::<bool>(), ext7(r));
    }),
    Test::new("xori", |e, r, _| {
        e.xori(r.random::<bool>(), r.random());
    }),
    Test::new("xorr", |e, r, _| {
        e.xorr(r.random::<bool>(), r.random::<bool>(), ext7(r));
    }),
    // Extended opcode tests.
    Test::new("dr", |e, r, _| e.nx(ext::Dr(r.random::<u8>()))),
    Test::new("ir", |e, r, _| e.nx(ext::Ir(r.random::<u8>()))),
    Test::new("mv", |e, r, _| {
        e.nx(ext::Mv(r.random::<u8>(), r.random::<u8>()))
    }),
    Test::new("nr", |e, r, _| e.nx(ext::Nr(r.random::<u8>()))),
    // load/store main operations
    Test::new("sr-lr", |e, _, _| {
        for i in 1..32 {
            e.sr(0, i);
            e.lr(i as u8, i);
        }
    }),
    Test::new("sr-out-of-bounds", |e, _, _| {
        e.sr(0, 0x2000);
    }),
    Test::new("lr-out-of-bounds", |e, _, _| {
        e.lr(0, 0x2000);
    }),
    Test::new("ilrr", |e, _, _| {
        for i in 1..32 {
            e.lri(regs::Ar0, i);
            e.ilrr(true, regs::Ar0);
            e.mrr(i as u8, regs::Ac1m);
        }
    }),
    Test::new("ilrr-out-of-bounds", |e, _, _| {
        e.lr(0, 0x1000);
    }),
    Test::new("ilrrd", |e, _, _| {
        e.lri(regs::Ar0, 31);
        for i in 1..32 {
            e.ilrrd(true, regs::Ar0);
            e.mrr(i as u8, regs::Ac1m);
        }
    }),
    Test::new("ilrrd-underflow", |e, _, _| {
        e.lri(regs::Ar0, 0);
        e.ilrrd(true, regs::Ar0);
    }),
    Test::new("ilrri", |e, _, _| {
        e.lri(regs::Ar0, 0);
        for i in 1..32 {
            e.ilrri(true, regs::Ar0);
            e.mrr(i as u8, regs::Ac1m);
        }
    }),
    Test::new("ilrri-overflow-out-of-bounds", |e, _, _| {
        e.lri(regs::Ar0, 0xffff);
        for i in 1..32 {
            e.ilrri(true, regs::Ar0);
            e.mrr(i as u8, regs::Ac1m);
        }
    }),
    Test::new("ilrrn", |e, _, _| {
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar1, 0);
        e.lri(regs::Ar2, 0);
        e.lri(regs::Ar3, 0);
        e.lri(regs::Ix0, 0);
        e.lri(regs::Ix1, 1);
        e.lri(regs::Ix2, 32);
        e.lri(regs::Ix3, 64);
        for i in 8..12 {
            e.ilrrn(true, i - 8);
            e.mrr(i, regs::Ac1m);
        }
    }),
    Test::new("ilrrn-overflow", |e, _, _| {
        e.lri(regs::Ar0, 1);
        e.lri(regs::Ix0, 0xffff);
        e.ilrrn(true, 0);
    }),
    Test::new("srr-lrr", |e, _, _| {
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar1, 1);
        e.lri(regs::Ar2, 2);
        e.lri(regs::Ar3, 3);
        for i in 4..32 {
            e.srr(i, i);
            e.lrr(i, i);
        }
    }),
    Test::new("srr-out-of-bounds", |e, _, _| {
        e.lri(regs::Ar0, 0x2000);
        e.srr(0, 0);
    }),
    Test::new("lrr-out-of-bounds", |e, _, _| {
        e.lri(regs::Ar0, 0x2000);
        e.lrr(0, 0);
    }),
    Test::new("srrd-lrrd", |e, _, _| {
        e.lri(regs::Ar0, 31);
        e.lri(regs::Ar1, 31);
        for i in 2..32 {
            e.srrd(regs::Ar0, i);
            e.lrrd(i, regs::Ar1);
        }
    }),
    Test::new("srrd-out-of-bounds", |e, _, _| {
        e.lri(regs::Ar0, 0x2000);
        e.srrd(0, 0);
    }),
    Test::new("srrd-underflow", |e, _, _| {
        e.lri(regs::Ar0, 0);
        e.srrd(0, 0);
    }),
    Test::new("lrrd-out-of-bounds", |e, _, _| {
        e.lri(regs::Ar0, 0x2000);
        e.lrrd(0, 0);
    }),
    Test::new("lrrd-underflow", |e, _, _| {
        e.lri(regs::Ar0, 0);
        e.lrrd(0, 0);
    }),
    Test::new("srri-lrri", |e, _, _| {
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar1, 0);
        for i in 2..32 {
            e.srri(regs::Ar0, i);
            e.lrri(i, regs::Ar1);
        }
    }),
    Test::new("srri-overflow-out-of-bounds", |e, _, _| {
        e.lri(regs::Ar0, 0xffff);
        e.srri(regs::Ar0, regs::Wr0);
    }),
    Test::new("lrri-overflow-out-of-bounds", |e, _, _| {
        e.lri(regs::Ar0, 0xffff);
        e.lrri(regs::Wr0, regs::Ar0);
    }),
    Test::new("srrn-lrrn", |e, _, _| {
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar1, 0);
        e.lri(regs::Ar2, 0);
        e.lri(regs::Ar3, 0);
        e.lri(regs::Ix0, 0);
        e.lri(regs::Ix1, 1);
        e.lri(regs::Ix2, 32);
        e.lri(regs::Ix3, 64);
        for i in 8..12 {
            e.srri(i, i);
            e.lrri(i, i);
        }
    }),
    Test::new("srrn-overflow", |e, _, _| {
        e.lri(regs::Ar0, 1);
        e.lri(regs::Ix0, 0xffff);
        e.srrn(0, 0);
    }),
    Test::new("lrrn-overflow", |e, _, _| {
        e.lri(regs::Ar0, 1);
        e.lri(regs::Ix0, 0xffff);
        e.lrrn(1, 0);
    }),
    // load/store extended operations
    Test::new("s-l", |e, r, _| {
        let ar = r.random::<u8>();
        let s = r.random::<u8>();
        e.lri(ar, r.random_range(0..0x1000));
        e.nx(ext::S(ar, s));
        e.mrr(regs::Ac0m, ar);
        e.decm(regs::Ac0m, ext::Nop);
        e.mrr(ar, regs::Ac0m);
        e.nx(ext::L(0, ar));
    }),
    Test::new("sn-ln", |e, r, _| {
        let ar: u8 = r.random_range(0..4);
        let s = r.random::<u8>();
        e.lri(ar, r.random_range(0..0x1000));
        // Load IX register
        e.lri(ar + 4, r.random_range(0..0x1000));
        e.nx(ext::Sn(ar, s));
        e.clr(false, ext::Nop);
        e.clr(true, ext::Nop);
        e.mrr(regs::Ac0m, ar);
        // Load IX register
        e.mrr(regs::Ac1m, ar + 4);
        e.sub(regs::Ac0, ext::Nop);
        e.mrr(ar, regs::Ac0m);
        e.nx(ext::Ln(0, ar));
    }),
    Test::new("ls", |e, r, _| {
        // copy over one value to another
        for _ in 0..4 {
            e.si(0, r.random());
            e.si(1, r.random());
            e.si(2, r.random());
            e.si(3, r.random());
        }
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar3, 4);
        for i in 0..4 {
            e.nx(ext::Ls(i, i % 2 != 0));
            e.mrr(20 + i, 30 + i % 2);
        }

        for i in 8..12 {
            e.lr(i, 4 + i as u16);
        }

        // read and store from same location
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar3, 0);
        e.lri(regs::Ax0l, r.random());
        e.lri(regs::Ac0m, r.random());
        e.nx(ext::Ls(0, false));
    }),
    Test::new("sl", |e, r, _| {
        // copy over one value to another
        for _ in 0..4 {
            e.si(0, r.random());
            e.si(1, r.random());
            e.si(2, r.random());
            e.si(3, r.random());
        }
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar3, 4);
        for i in 0..4 {
            e.nx(ext::Sl(i % 2 != 0, i));
            e.mrr(20 + i, 30 + i % 2);
        }

        for i in 8..12 {
            e.lr(i, 4 + i as u16);
        }

        // read and store from same location
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar3, 0);
        e.lri(regs::Ax0l, r.random());
        e.lri(regs::Ac0m, r.random());
        e.nx(ext::Sl(false, 0));
    }),
    Test::new("lsn", |e, r, _| {
        e.lri(regs::Ix0, r.random_range(0..64));
        // copy over one value to another
        for _ in 0..4 {
            e.si(0, r.random());
            e.si(1, r.random());
            e.si(2, r.random());
            e.si(3, r.random());
        }
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar3, 4);
        for i in 0..4 {
            e.nx(ext::Lsn(i, i % 2 != 0));
            e.mrr(20 + i, 30 + i % 2);
        }

        for i in 8..12 {
            e.lr(i, 4 + i as u16);
        }

        // read and store from same location
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar3, 0);
        e.lri(regs::Ax0l, r.random());
        e.lri(regs::Ac0m, r.random());
        e.nx(ext::Lsn(0, false));
    }),
    Test::new("sln", |e, r, _| {
        e.lri(regs::Ix0, r.random_range(0..64));
        // copy over one value to another
        for _ in 0..4 {
            e.si(0, r.random());
            e.si(1, r.random());
            e.si(2, r.random());
            e.si(3, r.random());
        }
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar3, 4);
        for i in 0..4 {
            e.nx(ext::Sln(i % 2 != 0, i));
            e.mrr(20 + i, 30 + i % 2);
        }

        for i in 8..12 {
            e.lr(i, 4 + i as u16);
        }

        // read and store from same location
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar3, 0);
        e.lri(regs::Ax0l, r.random());
        e.lri(regs::Ac0m, r.random());
        e.nx(ext::Sln(false, 0));
    }),
    Test::new("lsm", |e, r, _| {
        e.lri(regs::Ix3, r.random_range(0..64));
        // copy over one value to another
        for _ in 0..4 {
            e.si(0, r.random());
            e.si(1, r.random());
            e.si(2, r.random());
            e.si(3, r.random());
        }
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar3, 4);
        for i in 0..4 {
            e.nx(ext::Lsm(i, i % 2 != 0));
            e.mrr(20 + i, 30 + i % 2);
        }

        for i in 8..12 {
            e.lr(i, 4 + i as u16);
        }

        // read and store from same location
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar3, 0);
        e.lri(regs::Ax0l, r.random());
        e.lri(regs::Ac0m, r.random());
        e.nx(ext::Lsm(0, false));
    }),
    Test::new("slm", |e, r, _| {
        e.lri(regs::Ix3, r.random_range(0..64));
        // copy over one value to another
        for _ in 0..4 {
            e.si(0, r.random());
            e.si(1, r.random());
            e.si(2, r.random());
            e.si(3, r.random());
        }
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar3, 4);
        for i in 0..4 {
            e.nx(ext::Slm(i % 2 != 0, i));
            e.mrr(20 + i, 30 + i % 2);
        }

        for i in 8..12 {
            e.lr(i, 4 + i as u16);
        }

        // read and store from same location
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar3, 0);
        e.lri(regs::Ax0l, r.random());
        e.lri(regs::Ac0m, r.random());
        e.nx(ext::Slm(false, 0));
    }),
    Test::new("lsnm", |e, r, _| {
        e.lri(regs::Ix0, r.random_range(0..64));
        e.lri(regs::Ix3, r.random_range(0..64));
        // copy over one value to another
        for _ in 0..4 {
            e.si(0, r.random());
            e.si(1, r.random());
            e.si(2, r.random());
            e.si(3, r.random());
        }
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar3, 4);
        for i in 0..4 {
            e.nx(ext::Lsnm(i, i % 2 != 0));
            e.mrr(20 + i, 30 + i % 2);
        }

        for i in 8..12 {
            e.lr(i, 4 + i as u16);
        }

        // read and store from same location
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar3, 0);
        e.lri(regs::Ax0l, r.random());
        e.lri(regs::Ac0m, r.random());
        e.nx(ext::Lsnm(0, false));
    }),
    Test::new("slnm", |e, r, _| {
        e.lri(regs::Ix0, r.random_range(0..64));
        e.lri(regs::Ix3, r.random_range(0..64));
        // copy over one value to another
        for _ in 0..4 {
            e.si(0, r.random());
            e.si(1, r.random());
            e.si(2, r.random());
            e.si(3, r.random());
        }
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar3, 4);
        for i in 0..4 {
            e.nx(ext::Slnm(i % 2 != 0, i));
            e.mrr(20 + i, 30 + i % 2);
        }

        for i in 8..12 {
            e.lr(i, 4 + i as u16);
        }

        // read and store from same location
        e.lri(regs::Ar0, 0);
        e.lri(regs::Ar3, 0);
        e.lri(regs::Ax0l, r.random());
        e.lri(regs::Ac0m, r.random());
        e.nx(ext::Slnm(false, 0));
    }),
    Test::new("ld", |e, r, _| {
        let adr0 = r.random_range(0..255);
        let adr1 = r.random_range(0..255);
        let ar = r.random::<u8>();
        e.si(adr0, r.random());
        e.si(adr1, r.random());
        e.lri(ar, adr0 as u16);
        e.lri(regs::Ar3, adr1 as u16);
        e.nx(ext::Ld(r.random::<bool>(), r.random::<bool>(), ar));
    }),
    Test::new("ldax", |e, r, _| {
        let adr0 = r.random_range(0..255);
        let adr1 = r.random_range(0..255);
        let ar = r.random::<u8>();
        e.si(adr0, r.random());
        e.si(adr1, r.random());
        e.lri(ar, adr0 as u16);
        e.lri(regs::Ar3, adr1 as u16);
        e.nx(ext::Ldax(r.random::<bool>(), ar));
    }),
    Test::new("ldn", |e, r, _| {
        let adr0 = r.random_range(0..255);
        let adr1 = r.random_range(0..255);
        let ar = r.random::<u8>();
        e.si(adr0, r.random());
        e.si(adr1, r.random());
        e.lri(ar, adr0 as u16);
        e.lri(regs::Ar3, adr1 as u16);
        e.nx(ext::Ldn(r.random::<bool>(), r.random::<bool>(), ar));
    }),
    Test::new("ldaxn", |e, r, _| {
        let adr0 = r.random_range(0..255);
        let adr1 = r.random_range(0..255);
        let ar = r.random::<u8>();
        e.si(adr0, r.random());
        e.si(adr1, r.random());
        e.lri(ar, adr0 as u16);
        e.lri(regs::Ar3, adr1 as u16);
        e.nx(ext::Ldaxn(r.random::<bool>(), ar));
    }),
    Test::new("ldm", |e, r, _| {
        let adr0 = r.random_range(0..255);
        let adr1 = r.random_range(0..255);
        let ar = r.random::<u8>();
        e.si(adr0, r.random());
        e.si(adr1, r.random());
        e.lri(ar, adr0 as u16);
        e.lri(regs::Ar3, adr1 as u16);
        e.nx(ext::Ldm(r.random::<bool>(), r.random::<bool>(), ar));
    }),
    Test::new("ldaxm", |e, r, _| {
        let adr0 = r.random_range(0..255);
        let adr1 = r.random_range(0..255);
        let ar = r.random::<u8>();
        e.si(adr0, r.random());
        e.si(adr1, r.random());
        e.lri(ar, adr0 as u16);
        e.lri(regs::Ar3, adr1 as u16);
        e.nx(ext::Ldaxm(r.random::<bool>(), ar));
    }),
    Test::new("ldnm", |e, r, _| {
        let adr0 = r.random_range(0..255);
        let adr1 = r.random_range(0..255);
        let ar = r.random::<u8>();
        e.si(adr0, r.random());
        e.si(adr1, r.random());
        e.lri(ar, adr0 as u16);
        e.lri(regs::Ar3, adr1 as u16);
        e.nx(ext::Ldnm(r.random::<bool>(), r.random::<bool>(), ar));
    }),
    Test::new("ldaxnm", |e, r, _| {
        let adr0 = r.random_range(0..255);
        let adr1 = r.random_range(0..255);
        let ar = r.random::<u8>();
        e.si(adr0, r.random());
        e.si(adr1, r.random());
        e.lri(ar, adr0 as u16);
        e.lri(regs::Ar3, adr1 as u16);
        e.nx(ext::Ldaxnm(r.random::<bool>(), ar));
    }),
    Test::new("srs-lrs", |e, r, _| {
        e.lri(regs::Config, r.random_range(0..=0xf));
        let ofs = r.random();
        e.srs(r.random::<u8>(), ofs);
        e.lrs(r.random::<u8>(), ofs);
    }),
    Test::new("srsh-lrs", |e, r, _| {
        e.lri(regs::Config, r.random_range(0..=0xf));
        let ofs = r.random();
        e.srsh(r.random::<bool>(), ofs);
        e.lrs(r.random::<u8>(), ofs);
    }),
    // complex loop testing
    Test::new("loop-nested", |e, _, _| {
        e.set16(ext::Nop);

        // nested loop 1
        e.clr(regs::Ac1, ext::Nop);
        e.loopi(8);
        e.loopi(8);
        e.inc(regs::Ac1, ext::Nop);
        e.mrr(regs::Wr0, regs::Ac1l);

        // nested loop 2
        e.clr(regs::Ac1, ext::Nop);
        e.loopi(8);
        e.loopi(8);
        e.loopi(8);
        e.inc(regs::Ac1, ext::Nop);
        e.mrr(regs::Wr1, regs::Ac1l);

        // nested loop 3
        e.clr(regs::Ac1, ext::Nop);
        e.loopi(8);
        e.loopi(8);
        e.loopi(8);
        e.loopi(8);
        e.inc(regs::Ac1, ext::Nop);
        e.mrr(regs::Wr2, regs::Ac1l);

        // nested loop 4
        e.clr(regs::Ac1, ext::Nop);
        e.loopi(8);
        e.loopi(8);
        e.loopi(8);
        e.loopi(8);
        e.loopi(8);
        e.inc(regs::Ac1, ext::Nop);
        e.mrr(regs::Wr3, regs::Ac1l);
    }),
    // bloop testing
    Test::new("bloop-basic", |e, _, a| {
        let len_before = e.len();
        macro_rules! adr {
            () => {
                a + u16::try_from(e.len() - len_before).unwrap()
            };
        }

        e.jcc(Cond::Always, a + 3);
        let call_adr = adr!();
        e.inc(regs::Ac0, ext::Nop);
        e.ret(Cond::Always);

        e.set16(ext::Nop);

        // Bloop single instruction
        e.clr(regs::Ac0, ext::Nop);
        e.bloopi(32, adr!() + 2);
        e.inc(regs::Ac0, ext::Nop);

        e.mrr(regs::Ix0, regs::Ac0l);

        // Bloop multiple instructions
        e.clr(regs::Ac0, ext::Nop);
        e.clr(regs::Ac1, ext::Nop);
        e.bloopi(32, adr!() + 4);
        e.inc(regs::Ac0, ext::Nop);
        e.inc(regs::Ac1, ext::Nop);

        e.mrr(regs::Ix1, regs::Ac0l);
        e.mrr(regs::Ix2, regs::Ac1l);

        // Bloop call single
        e.clr(regs::Ac0, ext::Nop);
        e.bloopi(32, adr!() + 2);
        e.callcc(Cond::Always, call_adr);

        e.mrr(regs::Ix3, regs::Ac0l);

        // Bloop call with nop
        e.clr(regs::Ac0, ext::Nop);
        e.bloopi(32, adr!() + 3);
        e.callcc(Cond::Always, call_adr);
        e.nop();
    }),
    Test::new("bloop-nested", |e, _, a| {
        let len_before = e.len();
        macro_rules! adr {
            () => {
                a + u16::try_from(e.len() - len_before).unwrap()
            };
        }

        e.clr(regs::Ac0, ext::Nop);
        e.clr(regs::Ac1, ext::Nop);

        // Nested 0
        e.bloopi(32, adr!() + 2);
        e.bloopi(32, adr!() + 2);
        e.inc(regs::Ac0, ext::Nop);

        // nested 1
        e.bloopi(32, adr!() + 5);
        e.bloopi(32, adr!() + 2);
        e.inc(regs::Ac0, ext::Nop);

        // nested 2
        e.bloopi(32, adr!() + 6);
        e.bloopi(32, adr!() + 2);
        e.inc(regs::Ac0, ext::Nop);
        e.nop();
    }),
    Test::new("bloop-self", |e, _, a| {
        let len_before = e.len();
        macro_rules! adr {
            () => {
                a + u16::try_from(e.len() - len_before).unwrap()
            };
        }

        e.clr(regs::Ac0, ext::Nop);
        e.bloopi(32, adr!());
        e.inc(regs::Ac0, ext::Nop);
    }),
    Test::new("bloop-overflow", |e, _, a| {
        let len_before = e.len();
        macro_rules! adr {
            () => {
                a + u16::try_from(e.len() - len_before).unwrap()
            };
        }

        e.clr(regs::Ac0, ext::Nop);
        e.bloopi(32, adr!() + 10);
        e.bloopi(32, adr!() + 8);
        e.bloopi(32, adr!() + 6);
        e.bloopi(32, adr!() + 4);
        e.bloopi(32, adr!() + 2);
        e.inc(regs::Ac0, ext::Nop);
    }),
    Test::new("loop-bloop", |e, _, a| {
        let len_before = e.len();
        macro_rules! adr {
            () => {
                a + u16::try_from(e.len() - len_before).unwrap()
            };
        }

        e.clr(regs::Ac0, ext::Nop);
        e.loop_(32);
        e.bloopi(32, adr!() + 2);
        e.inc(regs::Ac0, ext::Nop);
    }),
    Test::new("bloop-loop", |e, _, a| {
        let len_before = e.len();
        macro_rules! adr {
            () => {
                a + u16::try_from(e.len() - len_before).unwrap()
            };
        }

        e.clr(regs::Ac0, ext::Nop);
        e.bloopi(32, adr!() + 2);
        e.loop_(32);
        e.inc(regs::Ac0, ext::Nop);
    }),
    // Test extended operations colliding with regular operations
    Test::new("ext-ops-extra", |e, r, _| {
        e.si(0, r.random());
        e.clr(regs::Ac0, ext::Nop);
        e.lri(regs::Ar0, 0);

        e.inc(regs::Ac0, ext::L(regs::Ac0l, regs::Ar0));
    }),
    // TODO:
    // - branching (jmp, call, ret, rti)
    // - exception specific tests
    // - interrupt testing
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

    let prologue_len = {
        let mut e: Emitter = Emitter::default();
        tools::dsp::test_prologue(&mut e, InputState::default());
        u16::try_from(e.len()).unwrap()
    };
    let epilogue_len = {
        let mut e: Emitter = Emitter::default();
        tools::dsp::test_epilogue(&mut e);
        u16::try_from(e.len()).unwrap()
    };

    let block_start_len = {
        let mut e: Emitter = Emitter::default();
        tools::dsp::block_start(&mut e);
        u16::try_from(e.len()).unwrap()
    };

    let mut binary_rules = "".to_string();
    let mut results_files = vec![];
    let mut total_bytes_of_tests = 0;
    for cur in TESTS {
        binary_rules += cur.name;
        binary_rules += ".bin ";

        println!("generating {}", cur.name);
        let mut e: Emitter = Emitter::default();

        let total_tests = u16::try_from(tools::dsp::GEN_DSP_INPUTS)?;
        e.emit(total_tests);
        assert!(cur.name.len() < 32);
        let mut name = [0u8; 32];
        for (i, b) in cur.name.as_bytes().iter().cloned().enumerate().take(31) {
            name[i] = b;
        }
        for b in name.iter().cloned().array_chunks::<2>() {
            e.emit(u16::from_be_bytes(b));
        }

        let mut adr = block_start_len;
        let mut assumed_size = None;
        for _ in 0..total_tests {
            if let Some(assumed_size) = assumed_size
                && adr + assumed_size > 0x1000
            {
                adr = block_start_len;
            }

            let size_start = adr;
            adr += prologue_len;

            let size_before = e.len();
            (cur.body)(&mut e, &mut rand, adr);
            // Emit a nop in the case the test runs an exception and needs to return to
            // the instruction after the fault.
            e.nop();

            adr += u16::try_from(e.len() - size_before).unwrap();
            adr += epilogue_len;

            if adr > 0x1000 {
                panic!("invalid assumption hueristic");
            }

            assumed_size = Some(assumed_size.unwrap_or(adr - size_start));

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
                .map(|s| format!(
                    "\t\"dvd:/{}\"",
                    PathBuf::from(s).file_name().unwrap().display()
                ))
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
