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

pub trait Reg16 {
    fn index(&self) -> u8;
}

impl Reg16 for u8 {
    #[inline(always)]
    fn index(&self) -> u8 {
        *self & 0xf
    }
}

pub trait RegAc {
    fn index(&self) -> u8;
}

impl RegAc for bool {
    #[inline(always)]
    fn index(&self) -> u8 {
        *self as u8
    }
}

pub trait RegAcLM {
    fn index(&self) -> u8;
}

impl RegAcLM for u8 {
    #[inline(always)]
    fn index(&self) -> u8 {
        *self & 3
    }
}

pub trait RegAcM {
    fn index(&self) -> u8;
}

impl RegAcM for bool {
    #[inline(always)]
    fn index(&self) -> u8 {
        *self as u8
    }
}

pub trait RegAcH {
    fn index(&self) -> u8;
}

impl RegAcH for bool {
    #[inline(always)]
    fn index(&self) -> u8 {
        *self as u8
    }
}

pub trait RegAr {
    fn index(&self) -> u8;
}

impl RegAr for u8 {
    fn index(&self) -> u8 {
        *self & 3
    }
}

pub trait RegIx {
    fn index(&self) -> u8;
}

impl RegIx for u8 {
    fn index(&self) -> u8 {
        *self & 3
    }
}

pub trait RegAx {
    fn index(&self) -> u8;
}

impl RegIx for bool {
    fn index(&self) -> u8 {
        *self as u8
    }
}

pub trait RegAxLH {
    fn index(&self) -> u8;
}

impl RegAxLH for u8 {
    fn index(&self) -> u8 {
        *self & 3
    }
}

pub trait RegAx0 {
    fn index(&self) -> u8;
}

impl RegAx0 for bool {
    fn index(&self) -> u8 {
        *self as u8
    }
}

pub trait RegAx1 {
    fn index(&self) -> u8;
}

impl RegAx1 for bool {
    fn index(&self) -> u8 {
        *self as u8
    }
}

pub trait RegAxH {
    fn index(&self) -> u8;
}

impl RegAxH for bool {
    fn index(&self) -> u8 {
        *self as u8
    }
}

pub mod regs {
    use super::*;

    macro_rules! regs {
        (
            $(
                $(!reg:$reg:expr,)?
                $(!reg16:$reg16:expr,)?
                $(!ac:$ac:expr,)?
                $(!aclm:$aclm:expr,)?
                $(!acm:$acm:expr,)?
                $(!ach:$ach:expr,)?
                $(!ar:$ar:expr,)?
                $(!ix:$ix:expr,)?
                $(!ax:$ax:expr,)?
                $(!ax0:$ax0:expr,)?
                $(!ax1:$ax1:expr,)?
                $(!axh:$axh:expr,)?
                $(!axlh:$axlh:expr,)?
                $(!alias:$alias:ident,)?
                $ident:ident;
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
                    impl Reg16 for $ident {
                        #[inline(always)]
                        fn index(&self) -> u8 {
                            $reg16
                        }
                    }
                )?

                $(
                    impl RegAc for $ident {
                        #[inline(always)]
                        fn index(&self) -> u8 {
                            $ac
                        }
                    }
                )?

                $(
                    impl RegAcLM for $ident {
                        #[inline(always)]
                        fn index(&self) -> u8 {
                            $aclm
                        }
                    }
                )?

                $(
                    impl RegAcM for $ident {
                        #[inline(always)]
                        fn index(&self) -> u8 {
                            $acm
                        }
                    }
                )?

                $(
                    impl RegAcH for $ident {
                        #[inline(always)]
                        fn index(&self) -> u8 {
                            $ach
                        }
                    }
                )?

                $(
                    impl RegAr for $ident {
                        #[inline(always)]
                        fn index(&self) -> u8 {
                            $ar
                        }
                    }
                )?

                $(
                    impl RegIx for $ident {
                        #[inline(always)]
                        fn index(&self) -> u8 {
                            $ix
                        }
                    }
                )?

                $(
                    impl RegAx for $ident {
                        #[inline(always)]
                        fn index(&self) -> u8 {
                            $ax
                        }
                    }
                )?

                $(
                    impl RegAx0 for $ident {
                        #[inline(always)]
                        fn index(&self) -> u8 {
                            $ax0
                        }
                    }
                )?

                $(
                    impl RegAx1 for $ident {
                        #[inline(always)]
                        fn index(&self) -> u8 {
                            $ax1
                        }
                    }
                )?

                $(
                    impl RegAxH for $ident {
                        #[inline(always)]
                        fn index(&self) -> u8 {
                            $axh
                        }
                    }
                )?

                $(
                    impl RegAxLH for $ident {
                        #[inline(always)]
                        fn index(&self) -> u8 {
                            $ax
                        }
                    }
                )?
            )*
        };
    }

    regs!(
        !reg:0, !reg16:0, !ar:0, !alias:R0, Ar0;
        !reg:1, !reg16:1, !ar:1, !alias:R1, Ar1;
        !reg:2, !reg16:2, !ar:2, !alias:R2, Ar2;
        !reg:3, !reg16:3, !ar:3, !alias:R3, Ar3;
        !reg:4, !reg16:4, !ix:0, !alias:R4, Ix0;
        !reg:5, !reg16:5, !ix:1, !alias:R5, Ix1;
        !reg:6, !reg16:6, !ix:2, !alias:R6, Ix2;
        !reg:7, !reg16:7, !ix:3, !alias:R7, Ix3;
        !reg:8, !reg16:8, !alias:R8, Wr0;
        !reg:9, !reg16:9, !alias:R9, Wr1;
        !reg:10, !reg16:10, !alias:R10, Wr2;
        !reg:11, !reg16:11, !alias:R11, Wr3;
        !reg:12, !reg16:12, !alias:R12, St0;
        !reg:13, !reg16:13, !alias:R13, St1;
        !reg:14, !reg16:14, !alias:R14, St2;
        !reg:15, !reg16:15, !alias:R15, St3;
        !reg:16, !ach:0, !alias:R16, Ac0h;
        !reg:17, !ach:1, !alias:R17, Ac1h;
        !reg:18, !alias:R18, Config;
        !reg:19, !alias:R19, Sr;
        !reg:20, !alias:R20, Prodl;
        !reg:21, !alias:R21, Prodm1;
        !reg:22, !alias:R22, Prodh;
        !reg:23, !alias:R23, Prodm2;
        !reg:24, !ax0:0, !axlh:0, !alias:R24, Ax0l;
        !reg:25, !ax1:0, !axlh:1, !alias:R25, Ax1l;
        !reg:26, !ax0:1, !axh:0, !axlh:2, !alias:R26, Ax0h;
        !reg:27, !ax1:1, !axh:1, !axlh:3, !alias:R27, Ax1h;
        !reg:28, !aclm:0, !alias:R28, Ac0l;
        !reg:29, !aclm:1, !alias:R29, Ac1l;
        !reg:30, !aclm:2, !acm:0, !alias:R30, Ac0m;
        !reg:31, !aclm:3, !acm:1, !alias:R31, Ac1m;
        !ac:0, Ac0;
        !ac:1, Ac1;
        !ax:0, Ax0;
        !ax:1, Ax1;
    );
}

pub trait ExtendedOpcode {
    fn extended_opc(self) -> u8;
}

pub trait ExtendedOpcode7 {
    fn extended_opc(self) -> u8;
}

impl ExtendedOpcode for u8 {
    fn extended_opc(self) -> u8 {
        self
    }
}

impl ExtendedOpcode7 for u8 {
    fn extended_opc(self) -> u8 {
        self & 0x7f
    }
}

pub mod ext {
    use super::*;

    pub struct Dr<R>(pub R);

    impl<R: RegAr> ExtendedOpcode for Dr<R> {
        fn extended_opc(self) -> u8 {
            0b0000_0100 | self.0.index()
        }
    }

    pub struct Ir<R>(pub R);

    impl<R: RegAr> ExtendedOpcode for Ir<R> {
        fn extended_opc(self) -> u8 {
            0b0000_1000 | self.0.index()
        }
    }

    pub struct L<D, S>(pub D, pub S);

    impl<D: RegAxLH, S: RegAr> ExtendedOpcode for L<D, S> {
        fn extended_opc(self) -> u8 {
            0b0100_0000 | (self.0.index() << 3) | self.1.index()
        }
    }

    pub struct Ln<D, S>(pub D, pub S);

    impl<D: RegAxLH, S: RegAr> ExtendedOpcode for Ln<D, S> {
        fn extended_opc(self) -> u8 {
            0b0100_0100 | (self.0.index() << 3) | self.1.index()
        }
    }

    pub struct Ld<D, R, S>(pub D, pub R, pub S);

    impl<D: RegAx0, R: RegAx1, S: RegAr> ExtendedOpcode for Ld<D, R, S> {
        fn extended_opc(self) -> u8 {
            0b1100_0000 | (self.0.index() << 5) | (self.1.index() << 4) | self.2.index()
        }
    }

    pub struct Ldm<D, R, S>(pub D, pub R, pub S);

    impl<D: RegAx0, R: RegAx1, S: RegAr> ExtendedOpcode for Ldm<D, R, S> {
        fn extended_opc(self) -> u8 {
            0b1100_1000 | (self.0.index() << 5) | (self.1.index() << 4) | self.2.index()
        }
    }

    pub struct Ldnm<D, R, S>(pub D, pub R, pub S);

    impl<D: RegAx0, R: RegAx1, S: RegAr> ExtendedOpcode for Ldnm<D, R, S> {
        fn extended_opc(self) -> u8 {
            0b1100_1100 | (self.0.index() << 5) | (self.1.index() << 4) | self.2.index()
        }
    }

    pub struct Ldn<D, R, S>(pub D, pub R, pub S);

    impl<D: RegAx0, R: RegAx1, S: RegAr> ExtendedOpcode for Ldn<D, R, S> {
        fn extended_opc(self) -> u8 {
            0b1100_0100 | (self.0.index() << 5) | (self.1.index() << 4) | self.2.index()
        }
    }

    pub struct Ldax<R, S>(pub R, pub S);

    impl<R: RegAx, S: RegAr> ExtendedOpcode for Ldax<R, S> {
        fn extended_opc(self) -> u8 {
            0b1100_0011 | (self.1.index() << 5) | (self.0.index() << 4)
        }
    }

    pub struct Ldaxm<R, S>(pub R, pub S);

    impl<R: RegAx, S: RegAr> ExtendedOpcode for Ldaxm<R, S> {
        fn extended_opc(self) -> u8 {
            0b1100_1011 | (self.1.index() << 5) | (self.0.index() << 4)
        }
    }

    pub struct Ldaxnm<R, S>(pub R, pub S);

    impl<R: RegAx, S: RegAr> ExtendedOpcode for Ldaxnm<R, S> {
        fn extended_opc(self) -> u8 {
            0b1100_1111 | (self.1.index() << 5) | (self.0.index() << 4)
        }
    }

    pub struct Ldaxn<R, S>(pub R, pub S);

    impl<R: RegAx, S: RegAr> ExtendedOpcode for Ldaxn<R, S> {
        fn extended_opc(self) -> u8 {
            0b1100_0111 | (self.1.index() << 5) | (self.0.index() << 4)
        }
    }

    pub struct Ls<D, S>(pub D, pub S);

    impl<D: RegAxLH, S: RegAcM> ExtendedOpcode for Ls<D, S> {
        fn extended_opc(self) -> u8 {
            0b1000_0000 | (self.0.index() << 4) | self.1.index()
        }
    }

    pub struct Lsm<D, S>(pub D, pub S);

    impl<D: RegAxLH, S: RegAcM> ExtendedOpcode for Lsm<D, S> {
        fn extended_opc(self) -> u8 {
            0b1000_1000 | (self.0.index() << 4) | self.1.index()
        }
    }

    pub struct Lsnm<D, S>(pub D, pub S);

    impl<D: RegAxLH, S: RegAcM> ExtendedOpcode for Lsnm<D, S> {
        fn extended_opc(self) -> u8 {
            0b1000_1100 | (self.0.index() << 4) | self.1.index()
        }
    }

    pub struct Lsn<D, S>(pub D, pub S);

    impl<D: RegAxLH, S: RegAcM> ExtendedOpcode for Lsn<D, S> {
        fn extended_opc(self) -> u8 {
            0b1000_0100 | (self.0.index() << 4) | self.1.index()
        }
    }

    pub struct Mv<D, S>(pub D, pub S);

    impl<D: RegAxLH, S: RegAcLM> ExtendedOpcode for Mv<D, S> {
        fn extended_opc(self) -> u8 {
            0b0001_0000 | (self.0.index() << 2) | self.1.index()
        }
    }

    pub struct Nop;

    impl ExtendedOpcode for Nop {
        fn extended_opc(self) -> u8 {
            0b0000_0000
        }
    }

    pub struct Nr<R>(pub R);

    impl<R: RegAr> ExtendedOpcode for Nr<R> {
        fn extended_opc(self) -> u8 {
            0b0000_1100 | self.0.index()
        }
    }

    pub struct S<D, S>(pub D, pub S);

    impl<D: RegAr, S_: RegAcLM> ExtendedOpcode for S<D, S_> {
        fn extended_opc(self) -> u8 {
            0b0010_0000 | (self.1.index() << 3) | self.0.index()
        }
    }

    pub struct Sl<D, S>(pub D, pub S);

    impl<D: RegAcM, S: RegAxLH> ExtendedOpcode for Sl<D, S> {
        fn extended_opc(self) -> u8 {
            0b1000_0010 | (self.0.index() << 4) | self.1.index()
        }
    }

    pub struct Slm<D, S>(pub D, pub S);

    impl<D: RegAcM, S: RegAxLH> ExtendedOpcode for Slm<D, S> {
        fn extended_opc(self) -> u8 {
            0b1000_1010 | (self.0.index() << 4) | self.1.index()
        }
    }

    pub struct Slnm<D, S>(pub D, pub S);

    impl<D: RegAcM, S: RegAxLH> ExtendedOpcode for Slnm<D, S> {
        fn extended_opc(self) -> u8 {
            0b1000_1110 | (self.0.index() << 4) | self.1.index()
        }
    }

    pub struct Sln<D, S>(pub D, pub S);

    impl<D: RegAcM, S: RegAxLH> ExtendedOpcode for Sln<D, S> {
        fn extended_opc(self) -> u8 {
            0b1000_0110 | (self.0.index() << 4) | self.1.index()
        }
    }

    pub struct Sn<D, S>(pub D, pub S);

    impl<D: RegAr, S: RegAcLM> ExtendedOpcode for Sn<D, S> {
        fn extended_opc(self) -> u8 {
            0b0010_0100 | (self.1.index() << 3) | self.0.index()
        }
    }
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

    fn e(&mut self, v: u16) {
        self.inner.push(v);
    }

    fn ed(&mut self, hi: u16, lo: u16) {
        self.inner.push(hi);
        self.inner.push(lo);
    }

    fn eext(&mut self, v: u8, ext: impl ExtendedOpcode) {
        self.e(((v as u16) << 8) | ext.extended_opc() as u16);
    }

    fn eext7(&mut self, v: u8, ext: impl ExtendedOpcode7, b: bool) {
        self.e(((v as u16) << 8) | ((b as u16) << 7) | ext.extended_opc() as u16);
    }

    pub fn emit(&mut self, v: u16) {
        self.e(v);
    }

    pub fn nop(&mut self) {
        self.e(0);
    }

    pub fn mrr(&mut self, d: impl Reg, s: impl Reg) {
        self.e(0b0001_1100_0000_0000 | ((d.index() as u16) << 5) | s.index() as u16)
    }

    pub fn lri(&mut self, d: impl Reg, i: u16) {
        self.ed(0b0000_0000_1000_0000 | d.index() as u16, i)
    }

    pub fn si(&mut self, m: u8, i: u16) {
        self.ed(0b0001_0110_0000_0000 | m as u16, i)
    }

    pub fn sr(&mut self, s: impl Reg, m: u16) {
        self.ed(0b0000_0000_1110_0000 | s.index() as u16, m)
    }

    pub fn lr(&mut self, d: impl Reg, m: u16) {
        self.ed(0b0000_0000_1100_0000 | d.index() as u16, m)
    }

    pub fn andf(&mut self, d: impl RegAc, i: u16) {
        self.ed(0b0000_0010_1010_0000 | ((d.index() as u16) << 8), i)
    }

    pub fn jcc(&mut self, c: Cond, a: u16) {
        self.ed(0b0000_0010_1001_0000 | c as u16, a)
    }

    pub fn callcc(&mut self, c: Cond, a: u16) {
        self.ed(0b0000_0010_1011_0000 | c as u16, a)
    }

    pub fn ret(&mut self, c: Cond) {
        self.e(0b0000_0010_1101_0000 | c as u16)
    }

    pub fn rti(&mut self, c: Cond) {
        self.e(0b0000_0010_1111_0000 | c as u16)
    }

    pub fn abs(&mut self, d: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b1010_0001 | (d.index() << 3), ext)
    }

    pub fn add(&mut self, d: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b0100_1100 | d.index(), ext);
    }

    pub fn addarn(&mut self, d: impl RegAr, s: impl RegIx) {
        self.e(0b0000_0000_0001_0000 | ((s.index() as u16) << 2) | d.index() as u16);
    }

    pub fn addax(&mut self, d: impl RegAr, s: impl RegIx, ext: impl ExtendedOpcode) {
        self.eext(0b0100_1000 | (s.index() << 1) | d.index(), ext);
    }

    pub fn addaxl(&mut self, d: impl RegAc, s: impl RegAx, ext: impl ExtendedOpcode) {
        self.eext(0b0111_0000 | (s.index() << 1) | d.index(), ext);
    }

    pub fn addi(&mut self, d: impl RegAc, imm: u16) {
        self.ed(0b0000_0010_0000_0000 | ((d.index() as u16) << 8), imm);
    }

    pub fn addis(&mut self, d: impl RegAc, imm: u8) {
        self.e(0b0000_0100_0000_0000 | ((d.index() as u16) << 8) | imm as u16);
    }

    pub fn addp(&mut self, d: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b0100_1110 | d.index(), ext);
    }

    pub fn addpaxz(&mut self, d: impl RegAc, s: impl RegAx, ext: impl ExtendedOpcode) {
        self.eext(0b1111_1000 | (s.index() << 1) | d.index(), ext);
    }

    pub fn addr(&mut self, d: impl RegAc, s: impl RegAxLH, ext: impl ExtendedOpcode) {
        self.eext(0b0100_0000 | (s.index() << 1) | d.index(), ext);
    }

    pub fn andc(&mut self, d: impl RegAcM, ext: impl ExtendedOpcode) {
        self.eext(0b0011_1100 | d.index(), ext);
    }

    pub fn andcf(&mut self, d: impl RegAcM, imm: u16) {
        self.ed(0b0000_0010_1100_0000 | ((d.index() as u16) << 8), imm);
    }

    pub fn andi(&mut self, d: impl RegAcM, imm: u16) {
        self.ed(0b0000_0010_0100_0000 | ((d.index() as u16) << 8), imm);
    }

    pub fn andr(&mut self, d: impl RegAcM, s: impl RegAxH, ext: impl ExtendedOpcode7) {
        self.eext7(0b0011_0100 | d.index() | (s.index() << 1), ext, false);
    }

    pub fn asl(&mut self, r: impl RegAc, i: u8) {
        self.e(0b0001_0100_1000_0000 | ((r.index() as u16) << 8) | (i as u16 & 0x3f));
    }

    pub fn asr(&mut self, r: impl RegAc, i: u8) {
        self.e(0b0001_0100_1100_0000 | ((r.index() as u16) << 8) | (i as u16 & 0x3f));
    }

    pub fn asrn(&mut self) {
        self.e(0b0000_0010_1101_1011);
    }

    pub fn asrnr(&mut self, d: impl RegAc, ext: impl ExtendedOpcode7) {
        self.eext7(0b0011_1110 | d.index(), ext, true);
    }

    pub fn asrnrx(&mut self, d: impl RegAc, s: impl RegAxH, ext: impl ExtendedOpcode7) {
        self.eext7(0b0011_1000 | (s.index() << 1) | d.index(), ext, true);
    }

    pub fn asr16(&mut self, r: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b0011_1000 | (r.index() << 3), ext);
    }

    pub fn bloop(&mut self, r: impl Reg, a: u16) {
        self.ed(0b0000_0000_0110_0000 | r.index() as u16, a)
    }

    pub fn bloopi(&mut self, i: u8, a: u16) {
        self.ed(0b0001_0001_0000_0000 | i as u16, a)
    }

    pub fn callr(&mut self, c: Cond, r: impl Reg16) {
        self.e(0b0001_0111_0001_0000 | ((r.index() as u16) << 5) | c as u16)
    }

    pub fn clr15(&mut self, ext: impl ExtendedOpcode) {
        self.eext(0b1000_1100, ext)
    }

    pub fn clr(&mut self, r: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b1000_0001 | (r.index() << 3), ext)
    }

    pub fn clrl(&mut self, r: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b1111_1100 | r.index(), ext)
    }

    pub fn clrp(&mut self, ext: impl ExtendedOpcode) {
        self.eext(0b1000_0100, ext)
    }

    pub fn cmp(&mut self, ext: impl ExtendedOpcode) {
        self.eext(0b1000_0010, ext)
    }

    pub fn cmpaxh(&mut self, s: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b1100_0001 | (s.index() << 3), ext)
    }

    pub fn cmpi(&mut self, d: impl RegAc, i: u16) {
        self.ed(0b0000_0010_1000_0000 | ((d.index() as u16) << 8), i)
    }

    pub fn cmpis(&mut self, d: impl RegAc, i: u8) {
        self.e(0b0000_0110_0000_0000 | ((d.index() as u16) << 8) | i as u16)
    }

    pub fn dar(&mut self, d: impl RegAr) {
        self.e(0b0000_0000_0000_0100 | d.index() as u16);
    }

    pub fn dec(&mut self, d: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b0111_1010 | d.index(), ext);
    }

    pub fn decm(&mut self, d: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b0111_1000 | d.index(), ext);
    }

    pub fn iar(&mut self, d: impl RegAr) {
        self.e(0b0000_0000_0000_1000 | d.index() as u16)
    }

    pub fn ifcc(&mut self, c: Cond) {
        self.e(0b0000_0010_0111_0000 | c as u16)
    }

    pub fn ilrr(&mut self, d: impl RegAcM, s: impl RegAr) {
        self.e(0b0000_0010_0001_0000 | ((d.index() as u16) << 8) | s.index() as u16)
    }

    pub fn ilrrd(&mut self, d: impl RegAcM, s: impl RegAr) {
        self.e(0b0000_0010_0001_0100 | ((d.index() as u16) << 8) | s.index() as u16)
    }

    pub fn ilrri(&mut self, d: impl RegAcM, s: impl RegAr) {
        self.e(0b0000_0010_0001_1000 | ((d.index() as u16) << 8) | s.index() as u16)
    }

    pub fn ilrrn(&mut self, d: impl RegAcM, s: impl RegAr) {
        self.e(0b0000_0010_0001_1100 | ((d.index() as u16) << 8) | s.index() as u16)
    }

    pub fn inc(&mut self, d: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b0111_0110 | d.index(), ext)
    }

    pub fn incm(&mut self, d: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b0111_0100 | d.index(), ext)
    }

    pub fn jrcc(&mut self, c: Cond, r: impl Reg16) {
        self.e(0b0001_0111_0000_0000 | ((r.index() as u16) << 5) | c as u16);
    }

    pub fn loop_(&mut self, r: impl Reg) {
        self.e(0b0000_0000_0100_0000 | r.index() as u16)
    }

    pub fn loopi(&mut self, i: u8) {
        self.e(0b0001_0000_0000_0000 | i as u16)
    }

    pub fn lris(&mut self, d: impl RegAxLH, i: u8) {
        self.e(0b0000_1000_0000_0000 | ((d.index() as u16) << 8) | i as u16)
    }

    pub fn lrr(&mut self, d: impl RegAxLH, s: impl RegAr) {
        self.e(0b0001_1000_0000_0000 | ((s.index() as u16) << 5) | d.index() as u16)
    }

    pub fn lrrd(&mut self, d: impl RegAxLH, s: impl RegAr) {
        self.e(0b0001_1000_1000_0000 | ((s.index() as u16) << 5) | d.index() as u16)
    }

    pub fn lrri(&mut self, d: impl RegAxLH, s: impl RegAr) {
        self.e(0b0001_1001_0000_0000 | ((s.index() as u16) << 5) | d.index() as u16)
    }

    pub fn lrrn(&mut self, d: impl RegAxLH, s: impl RegAr) {
        self.e(0b0001_1001_1000_0000 | ((s.index() as u16) << 5) | d.index() as u16)
    }

    pub fn lrs(&mut self, d: impl RegAxLH, m: u8) {
        self.e(0b0010_0000_0000_0000 | ((d.index() as u16) << 8) | m as u16)
    }

    pub fn lsl(&mut self, d: impl RegAc, i: u8) {
        self.e(0b0001_0100_0000_0000 | ((d.index() as u16) << 8) | (i as u16 & 0x3f))
    }

    pub fn lsl16(&mut self, r: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b1111_0000 | r.index(), ext)
    }

    pub fn lsr(&mut self, r: impl RegAc, i: u8) {
        self.e(0b0001_0100_0100_0000 | ((r.index() as u16) << 8) | (i as u16 & 0x3f))
    }

    pub fn lsrn(&mut self) {
        self.e(0b0000_0010_1100_1010)
    }

    pub fn lsrnr(&mut self, d: impl RegAc, ext: impl ExtendedOpcode7) {
        self.eext7(0b0011_1100 | d.index(), ext, true);
    }

    pub fn lsrnrx(&mut self, d: impl RegAc, s: impl RegAxH, ext: impl ExtendedOpcode7) {
        self.eext7(0b0011_0100 | (s.index() << 1) | d.index(), ext, true);
    }

    pub fn lsr16(&mut self, r: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b1111_0100 | r.index(), ext);
    }

    pub fn m0(&mut self, ext: impl ExtendedOpcode) {
        self.eext(0b1000_1011, ext);
    }

    pub fn m2(&mut self, ext: impl ExtendedOpcode) {
        self.eext(0b1000_1010, ext);
    }

    pub fn madd(&mut self, s: impl RegAx, ext: impl ExtendedOpcode) {
        self.eext(0b1111_0010 | s.index(), ext);
    }

    pub fn maddc(&mut self, s: impl RegAcM, t: impl RegAxH, ext: impl ExtendedOpcode) {
        self.eext(0b1110_1000 | (s.index() << 1) | t.index(), ext);
    }

    pub fn maddx(&mut self, s: impl RegAcM, t: impl RegAxH, ext: impl ExtendedOpcode) {
        self.eext(0b1110_0000 | (s.index() << 1) | t.index(), ext);
    }

    pub fn mov(&mut self, d: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b0110_1100 | d.index(), ext);
    }

    pub fn movax(&mut self, d: impl RegAc, s: impl RegAx, ext: impl ExtendedOpcode) {
        self.eext(0b0110_1000 | (s.index() << 1) | d.index(), ext);
    }

    pub fn movnp(&mut self, d: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b111_1110 | d.index(), ext)
    }

    pub fn movp(&mut self, d: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b0110_1110 | d.index(), ext);
    }

    pub fn movpz(&mut self, d: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b1111_1110 | d.index(), ext);
    }

    pub fn movr(&mut self, d: impl RegAc, s: impl RegAxLH, ext: impl ExtendedOpcode) {
        self.eext(0b0110_0000 | (s.index() << 1) | d.index(), ext);
    }

    pub fn msub(&mut self, s: impl RegAx, ext: impl ExtendedOpcode) {
        self.eext(0b1111_0110 | s.index(), ext)
    }

    pub fn msubc(&mut self, s: impl RegAc, t: impl RegAx, ext: impl ExtendedOpcode) {
        self.eext(0b1110_1100 | (s.index() << 1) | t.index(), ext);
    }

    pub fn msubx(&mut self, s: impl RegAx0, t: impl RegAx1, ext: impl ExtendedOpcode) {
        self.eext(0b1110_0100 | (s.index() << 1) | t.index(), ext);
    }

    pub fn mul(&mut self, s: impl RegAx, ext: impl ExtendedOpcode) {
        self.eext(0b1001_0000 | (s.index() << 3), ext);
    }

    pub fn mulac(&mut self, s: impl RegAx, r: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b1001_0100 | (s.index() << 3) | r.index(), ext)
    }

    pub fn mulaxh(&mut self, ext: impl ExtendedOpcode) {
        self.eext(0b1000_0011, ext);
    }

    pub fn mulc(&mut self, s: impl RegAcM, t: impl RegAxH, ext: impl ExtendedOpcode) {
        self.eext(0b1100_0000 | (s.index() << 4) | (t.index() << 3), ext)
    }

    pub fn mulcac(
        &mut self,
        s: impl RegAcM,
        t: impl RegAxH,
        r: impl RegAc,
        ext: impl ExtendedOpcode,
    ) {
        self.eext(
            0b1100_0100 | (s.index() << 4) | (t.index() << 3) | r.index(),
            ext,
        );
    }

    pub fn mulcmv(
        &mut self,
        s: impl RegAcM,
        t: impl RegAxH,
        r: impl RegAc,
        ext: impl ExtendedOpcode,
    ) {
        self.eext(
            0b1110_0110 | (s.index() << 4) | (t.index() << 3) | r.index(),
            ext,
        );
    }

    pub fn mulcmvz(
        &mut self,
        s: impl RegAcM,
        t: impl RegAxH,
        r: impl RegAc,
        ext: impl ExtendedOpcode,
    ) {
        self.eext(
            0b1100_0010 | (s.index() << 4) | (t.index() << 3) | r.index(),
            ext,
        );
    }

    pub fn mulmv(&mut self, s: impl RegAx, r: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b1001_0110 | (s.index() << 3) | r.index(), ext);
    }

    pub fn mulmvz(&mut self, s: impl RegAx, r: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b1001_0010 | (s.index() << 3) | r.index(), ext);
    }

    pub fn mulx(&mut self, s: impl RegAx0, t: impl RegAx1, ext: impl ExtendedOpcode) {
        self.eext(0b1010_0000 | (s.index() << 4) | (t.index() << 3), ext)
    }

    pub fn mulxac(
        &mut self,
        s: impl RegAx0,
        t: impl RegAx1,
        r: impl RegAc,
        ext: impl ExtendedOpcode,
    ) {
        self.eext(
            0b1010_0100 | (s.index() << 4) | (t.index() << 3) | r.index(),
            ext,
        )
    }

    pub fn mulxmv(
        &mut self,
        s: impl RegAx0,
        t: impl RegAx1,
        r: impl RegAc,
        ext: impl ExtendedOpcode,
    ) {
        self.eext(
            0b1010_0110 | (s.index() << 4) | (t.index() << 3) | r.index(),
            ext,
        )
    }

    pub fn mulxmvz(
        &mut self,
        s: impl RegAx0,
        t: impl RegAx1,
        r: impl RegAc,
        ext: impl ExtendedOpcode,
    ) {
        self.eext(
            0b1010_0010 | (s.index() << 4) | (t.index() << 3) | r.index(),
            ext,
        )
    }

    pub fn neg(&mut self, d: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b0111_1100 | d.index(), ext)
    }

    pub fn not(&mut self, d: impl RegAcM, ext: impl ExtendedOpcode7) {
        self.eext7(0b0011_0010 | d.index(), ext, true);
    }

    pub fn nx(&mut self, ext: impl ExtendedOpcode) {
        self.eext(0b1000_0000, ext);
    }

    pub fn orc(&mut self, d: impl RegAcM, ext: impl ExtendedOpcode7) {
        self.eext7(0b0011_1110 | d.index(), ext, false);
    }

    pub fn ori(&mut self, d: impl RegAcM, i: u16) {
        self.ed(0b0000_0010_0110_0000 | ((d.index() as u16) << 8), i)
    }

    pub fn orr(&mut self, d: impl RegAcM, s: impl RegAxH, ext: impl ExtendedOpcode7) {
        self.eext7(0b0011_1000 | (s.index() << 1) | d.index(), ext, false);
    }

    pub fn sbclr(&mut self, i: u8) {
        self.e(0b0001_0010_0000_0000 | (i as u16 & 0b111));
    }

    pub fn sbset(&mut self, i: u8) {
        self.e(0b0001_0011_0000_0000 | (i as u16 & 0b111));
    }

    pub fn set15(&mut self, ext: impl ExtendedOpcode) {
        self.eext(0b1000_1101, ext);
    }

    pub fn set16(&mut self, ext: impl ExtendedOpcode) {
        self.eext(0b1000_1110, ext);
    }

    pub fn set40(&mut self, ext: impl ExtendedOpcode) {
        self.eext(0b1000_1110, ext);
    }

    pub fn srr(&mut self, d: impl RegAr, s: impl Reg) {
        self.e(0b0001_1010_0000_0000 | ((d.index() as u16) << 5) | s.index() as u16)
    }

    pub fn srrd(&mut self, d: impl RegAr, s: impl Reg) {
        self.e(0b0001_1010_1000_0000 | ((d.index() as u16) << 5) | s.index() as u16)
    }

    pub fn srri(&mut self, d: impl RegAr, s: impl Reg) {
        self.e(0b0001_1011_0000_0000 | ((d.index() as u16) << 5) | s.index() as u16)
    }

    pub fn srrn(&mut self, d: impl RegAr, s: impl Reg) {
        self.e(0b0001_1011_1000_0000 | ((d.index() as u16) << 5) | s.index() as u16)
    }

    pub fn srs(&mut self, s: impl RegAcLM, m: u8) {
        self.e(0b0010_1100_0000_0000 | ((s.index() as u16) << 8) | m as u16)
    }

    pub fn srsh(&mut self, s: impl RegAcH, m: u8) {
        self.e(0b0010_1000_0000_0000 | ((s.index() as u16) << 8) | m as u16)
    }

    pub fn sub(&mut self, d: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b0101_1100 | d.index(), ext);
    }

    pub fn subarn(&mut self, d: impl RegAr) {
        self.e(0b0000_0000_0000_1100 | d.index() as u16)
    }

    pub fn subax(&mut self, d: impl RegAc, s: impl RegAx, ext: impl ExtendedOpcode) {
        self.eext(0b0101_1000 | (s.index() << 1) | d.index(), ext)
    }

    pub fn subp(&mut self, d: impl RegAc, s: impl RegAx, ext: impl ExtendedOpcode) {
        self.eext(0b0101_1000 | (s.index() << 1) | d.index(), ext)
    }

    pub fn subr(&mut self, d: impl RegAc, s: impl RegAxLH, ext: impl ExtendedOpcode) {
        self.eext(0b0101_1000 | (s.index() << 1) | d.index(), ext)
    }

    pub fn tst(&mut self, r: impl RegAc, ext: impl ExtendedOpcode) {
        self.eext(0b1011_0001 | (r.index() << 3), ext)
    }

    pub fn tstaxh(&mut self, r: impl RegAxH, ext: impl ExtendedOpcode) {
        self.eext(0b1000_0110 | r.index(), ext)
    }

    pub fn tstprod(&mut self, ext: impl ExtendedOpcode) {
        self.eext(0b1000_0101, ext);
    }

    pub fn xorc(&mut self, d: impl RegAcM, ext: impl ExtendedOpcode7) {
        self.eext7(0b0011_0000 | d.index(), ext, true);
    }

    pub fn xori(&mut self, d: impl RegAcM, i: u16) {
        self.ed(0b0000_0010_0010_0000 | ((d.index() as u16) << 8), i)
    }

    pub fn xorr(&mut self, d: impl RegAcM, s: impl RegAxH, ext: impl ExtendedOpcode7) {
        self.eext7(0b0011_0000 | (s.index() << 1) | d.index(), ext, false);
    }

    /// Used as a market or 'null' character for end of stream.
    pub fn emit_invalid_mark(&mut self) {
        self.e(0b0000_0000_1010_0000);
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
