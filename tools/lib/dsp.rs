use crate::dspemit::{
    Cond, Emitter, ext,
    regs::{self, Ac0, Ac0m, St0},
};
use rand::Rng;

pub const GEN_DSP_INPUTS: usize = 100;

#[derive(Default)]
pub struct InputState {
    pub regs: [u16; 32],
}

impl InputState {
    pub fn random() -> Self {
        Self {
            regs: std::array::from_fn(|_| rand::random()),
        }
    }

    pub fn random_with_rng(rng: &mut rand::rngs::SmallRng) -> Self {
        Self {
            regs: std::array::from_fn(|_| rng.random()),
        }
    }
}

pub fn serialize_output(e: &mut Emitter) {
    e.set16(ext::Nop);
    e.callcc(Cond::Always, 0x12);
    for i in 0..31 {
        e.mrr(regs::R31 {}, i);
        e.callcc(Cond::Always, 0x12);
    }
}

pub fn block_start(e: &mut Emitter) {
    for i in 0..8 {
        e.jcc(Cond::Always, 0x20 + i * 6);
    }

    // True start
    e.jcc(Cond::Always, 0xb2);

    // 0x12 : write r31 to cpu
    e.si(0xfc, 0);
    e.sr(31, 0xfffd);
    e.lr(31, 0xfffc);
    e.andf(true, 1 << 15);
    e.jcc(Cond::Lnz, (u16::try_from(e.len()).unwrap()).wrapping_sub(4));
    e.ret(Cond::Always);

    if e.len() >= 0x20 {
        panic!("should never happen");
    }

    while e.len() < 0x20 {
        e.nop();
    }

    for i in 0..8 {
        e.lri(i, 0xbeef);
        e.mrr(Ac0m, St0);
        e.addis(Ac0, 2);
        e.mrr(St0, Ac0m);
        e.rti(Cond::Always);
    }

    if e.len() > 0x50 {
        panic!("should never happen");
    }

    while e.len() < 0x50 {
        e.nop();
    }

    serialize_output(e);
    e.ret(Cond::Always);

    if e.len() >= 0xb2 {
        panic!("should never happen");
    }

    while e.len() < 0xb2 {
        e.nop();
    }
}

pub fn block_end(e: &mut Emitter) {
    // Clear 40-bit mode if enabled.
    e.set16(ext::Nop);
    // Jump back to IROM
    e.jcc(Cond::Always, 0x8000);
}

/// Insert test prelude
pub fn test_prologue(e: &mut Emitter, input: InputState) {
    for (i, &input) in input.regs.iter().enumerate() {
        e.lri(i as u8, input);
    }
}

/// Insert test epilogue
pub fn test_epilogue(e: &mut Emitter) {
    // TODO: restore this call
    // // Call serialize output
    // e.callcc(Cond::Always, 0x50);
    serialize_output(e);
}
