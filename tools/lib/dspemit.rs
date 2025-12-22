#[derive(Clone, Copy)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum Cond {
    Ge,
    L,
    G,
    Le,
    Nz,
    Z,
    Nc,
    C,
    x8,
    x9,
    xA,
    xB,
    Lnz,
    Lz,
    O,
    Always,
}

pub trait Reg {
    fn index(&self) -> u8;
}

impl Reg for u8 {
    #[inline(always)]
    fn index(&self) -> u8 {
        *self & 0x1f
    }
}

pub trait RegAdM {
    fn index(&self) -> u8;
}

impl RegAdM for bool {
    #[inline(always)]
    fn index(&self) -> u8 {
        *self as u8
    }
}

pub mod regs {
    use super::*;

    macro_rules! regs {
        (
            $(
            $(!reg:$reg:expr,)? $(!adm:$adm:expr,)? $(!alias:$alias:ident,)? $ident:ident;
            )*
        ) => {
            $(
                #[derive(Clone, Copy)]
                pub struct $ident;
                $(pub type $alias = $ident;)?

                $(
                    impl Reg for $ident {
                        #[inline(always)]
                        fn index(&self) -> u8 {
                            $reg
                        }
                    }
                )?

                $(
                    impl RegAdM for $ident {
                        #[inline(always)]
                        fn index(&self) -> u8 {
                            $adm
                        }
                    }
                )?
            )*
        };
    }

    regs!(
        !reg:0, !alias:R0, Ar0;
        !reg:1, !alias:R1, Ar1;
        !reg:2, !alias:R2, Ar2;
        !reg:3, !alias:R3, Ar3;
        !reg:4, !alias:R4, Ix0;
        !reg:5, !alias:R5, Ix1;
        !reg:6, !alias:R6, Ix2;
        !reg:7, !alias:R7, Ix3;
        !reg:8, !alias:R8, Wr0;
        !reg:9, !alias:R9, Wr1;
        !reg:10, !alias:R10, Wr2;
        !reg:11, !alias:R11, Wr3;
        !reg:12, !alias:R12, St0;
        !reg:13, !alias:R13, St1;
        !reg:14, !alias:R14, St2;
        !reg:15, !alias:R15, St3;
        !reg:16, !alias:R16, Ac0h;
        !reg:17, !alias:R17, Ac1h;
        !reg:18, !alias:R18, Config;
        !reg:19, !alias:R19, Sr;
        !reg:20, !alias:R20, Prodl;
        !reg:21, !alias:R21, Prodm1;
        !reg:22, !alias:R22, Prodh;
        !reg:23, !alias:R23, Prodm2;
        !reg:24, !alias:R24, Ax0l;
        !reg:25, !alias:R25, Ax1l;
        !reg:26, !alias:R26, Ax0h;
        !reg:27, !alias:R27, Ax1h;
        !reg:28, !alias:R28, Ac0l;
        !reg:29, !alias:R29, Ac1l;
        !reg:30, !adm:0, !alias:R30, Ac0m;
        !reg:31, !adm:1, !alias:R31, Ac1m;
    );
}

#[derive(Default)]
pub struct Emitter {
    inner: Vec<u16>,
}

impl Emitter {
    pub fn into_inner(self) -> Vec<u16> {
        self.inner
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn drain(&mut self, pos: usize) -> Vec<u16> {
        self.inner.drain(..pos).collect()
    }

    pub fn drain_into_emitter(&mut self, pos: usize) -> Self {
        Self {
            inner: self.drain(pos),
        }
    }

    pub fn append(&mut self, other: &mut Self) {
        self.inner.append(&mut other.inner);
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn e16(&mut self, v: u16) {
        self.inner.push(v);
    }

    fn e32(&mut self, v: u32) {
        let lo = v as u16;
        let hi = (v >> 16) as u16;
        self.inner.push(hi);
        self.inner.push(lo);
    }

    pub fn lri(&mut self, d: impl Reg, i: u16) {
        self.e32(((0b0000_0000_1000_0000 | d.index() as u32) << 16) | i as u32)
    }

    pub fn si(&mut self, m: u8, i: u16) {
        self.e32(((0b0001_0110_0000_0000 | m as u32) << 16) | i as u32)
    }

    pub fn sr(&mut self, s: impl Reg, m: u16) {
        self.e32(((0b0000_0000_1110_0000 | s.index() as u32) << 16) | m as u32)
    }

    pub fn lr(&mut self, d: u8, m: u16) {
        self.e32(((0b0000_0000_1100_0000 | (d as u32 & 0x1f)) << 16) | m as u32)
    }

    pub fn andf(&mut self, d: bool, i: u16) {
        self.e32(((0b0000_0010_1010_0000 | ((d as u32) << 8)) << 16) | i as u32)
    }

    pub fn jcc(&mut self, c: Cond, a: u16) {
        self.e32(((0b0000_0000_1100_0000 | (c as u32 & 0x0f)) << 16) | a as u32)
    }

    pub fn nop(&mut self) {
        self.e16(0);
    }
}

impl IntoIterator for Emitter {
    type Item = u16;
    type IntoIter = std::vec::IntoIter<u16>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a Emitter {
    type Item = &'a u16;
    type IntoIter = std::slice::Iter<'a, u16>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.as_slice().iter()
    }
}
