#include <dspemit/dspemit.h>
#include <stdint.h>

#define nibbles4(n0, n1, n2, n3)                                               \
  (((n0) << 12) | ((n1) << 8) | ((n2) << 4) | (n3))

#define nibbles2(n0, n1) (((n0) << 12) | ((n1) << 8))

#define enibbles(n2, n3) (((n2) << 4) | (n3))

// ----------- Ext instructions -----------

uint8_t emite_dr(uint8_t r) { return 0b0100 | (r & 3); }

uint8_t emite_ir(uint8_t r) { return 0b1000 | (r & 3); }

uint8_t emite_l(uint8_t d, uint8_t s) {
  return enibbles(0b0100 | ((d >> 1) & 3), ((d & 1) << 3) | (s & 3));
}

uint8_t emite_ln(uint8_t d, uint8_t s) {
  return enibbles(0b0100 | ((d >> 1) & 3), 0b0100 | ((d & 1) << 3) | (s & 3));
}

uint8_t emite_ld(bool d, bool r, uint8_t s) {
  return enibbles(0b1100 | (d << 1) | r, s & 3);
}

uint8_t emite_ldm(bool d, bool r, uint8_t s) {
  return enibbles(0b1100 | (d << 1) | r, 0b1000 | (s & 3));
}

uint8_t emite_ldnm(bool d, bool r, uint8_t s) {
  return enibbles(0b1100 | (d << 1) | r, 0b1100 | (s & 3));
}

uint8_t emite_ldn(bool d, bool r, uint8_t s) {
  return enibbles(0b1100 | (d << 1) | r, 0b0100 | (s & 3));
}

uint8_t emite_ls(uint8_t d, bool s) { return enibbles(0b1000 | (d & 3), s); }

uint8_t emite_lsm(uint8_t d, bool s) {
  return enibbles(0b1000 | (d & 3), 0b1000 | s);
}

uint8_t emite_lsnm(uint8_t d, bool s) {
  return enibbles(0b1000 | (d & 3), 0b1100 | s);
}

uint8_t emite_lsn(uint8_t d, bool s) {
  return enibbles(0b1000 | (d & 3), 0b0100 | s);
}

uint8_t emite_mv(uint8_t d, uint8_t s) {
  return enibbles(0b0001, ((d & 3) << 2) | (s & 3));
}

uint8_t emite_nr(uint8_t r) { return enibbles(0, 0b1100 | (r & 3)); }

uint8_t emite_s(uint8_t s, uint8_t d) {
  return enibbles(0b0010 | ((s >> 1) & 1), ((s & 1) << 3) | (d & 3));
}

uint8_t emite_sl(uint8_t d, bool s) {
  return enibbles(0b1000 | (d & 3), 0b0010 | s);
}

uint8_t emite_slm(uint8_t d, bool s) {
  return enibbles(0b1000 | (d & 3), 0b1010 | s);
}

uint8_t emite_slnm(uint8_t d, bool s) {
  return enibbles(0b1000 | (d & 3), 0b1110 | s);
}

uint8_t emite_sln(uint8_t d, bool s) {
  return enibbles(0b1000 | (d & 3), 0b0110 | s);
}

uint8_t emite_sn(uint8_t s, uint8_t d) {
  return enibbles(0b0010 | ((s >> 1) & 1), 0b0100 | ((s & 1) << 3) | (d & 3));
}

// ----------- Main instructions -----------

uint16_t emit_abs(bool d) { return nibbles2(0b1010, 0b001 | (d << 3)); }

uint16_t emit_add(bool d) { return nibbles2(0b0100, 1100 | d); }

uint16_t emit_addarn(uint16_t s, uint16_t d) {
  return nibbles4(0, 0, 1, ((s & 3) << 2) | (d & 3));
}

uint16_t emit_addax(bool s, bool d) {
  return nibbles2(0b0100, 0b1000 | (s << 1) | d);
}

uint16_t emit_addaxl(bool s, bool d) {
  return nibbles2(0b0111, 0b0000 | (s << 1) | d);
}

uint32_t emit_addi(bool d, uint16_t i) {
  return ((uint32_t)nibbles2(0, 0b0010 | d) << 16) | i;
}

uint16_t emit_addis(bool d, uint8_t i) { return nibbles2(0, 0b0100 | d) | i; }

uint16_t emit_addp(bool d) { return nibbles2(0b0100, 0b1110 | d); }

uint16_t emit_addpaxz(bool s, bool d) {
  return nibbles2(0b1111, 0b1000 | (s << 1) | d);
}

uint16_t emit_addr(uint16_t s, bool d) {
  return nibbles2(0b0100, ((s & 2) << 1) | d);
}

uint16_t emit_andc(bool d) { return nibbles2(0b0011, 0b1100 | d); }

uint32_t emit_andcf(bool d, uint16_t i) {
  return ((uint32_t)nibbles4(0, 0b0010 | d, 0b1100, 0) << 16) | i;
}

uint32_t emit_andf(bool d, uint16_t i) {
  return ((uint32_t)nibbles4(0, 0b0010 | d, 0b1010, 0) << 16) | i;
}

uint32_t emit_andi(bool d, uint16_t i) {
  return ((uint32_t)nibbles4(0, 0b0010 | d, 0b1000, 0) << 16) | i;
}

uint16_t emit_andr(bool s, bool d) {
  return nibbles2(0b0011, 0b0100 | (s << 1) | d);
}

uint16_t emit_asl(bool r, uint8_t i) {
  return nibbles4(0b0001, 0b0100 | r, 0b1000 | ((i >> 4) & 3), i & 15);
}

uint16_t emit_asr(bool r, uint8_t i) {
  return nibbles4(0b0001, 0b0100 | r, 0b1100 | ((i >> 4) & 3), i & 15);
}

uint16_t emit_asrn(void) { return nibbles4(0, 0b0010, 0b1101, 0b1011); }

uint16_t emit_asrnr(bool d) { return nibbles4(0b0011, 0b1110 | d, 0b1000, 0); }

uint16_t emit_asrnrx(bool s, bool d) {
  return nibbles4(0b0011, 0b1000 | (s << 1) | d, 0b1000, 0);
}

uint16_t emit_asr16(bool r) { return nibbles2(0b1001, 0b0001 | (r << 3)); }

uint32_t emit_bloop(uint8_t r, uint16_t a) {
  return ((uint32_t)nibbles4(0, 0, 0b0110 | ((r >> 4) & 1), r & 15) << 16) | a;
}

uint32_t emit_bloopi(uint8_t i, uint16_t a) {
  return (((uint32_t)nibbles2(0b0001, 0b0001) | i) << 16) | a;
}

uint32_t emit_call(uint16_t a) {
  return ((uint32_t)nibbles4(0, 0b0010, 0b10111, 0b1111) << 16) | a;
}

uint32_t emit_callcc(uint8_t c, uint16_t a) {
  return ((uint32_t)nibbles4(0, 0b0010, 0b10111, c & 15) << 16) | a;
}

uint16_t emit_callr(uint8_t r) {
  return nibbles4(0b0001, 0b0111, 0b0001 | ((r & 7) << 1), 0b1111);
}

uint16_t emit_callrcc(uint8_t r, uint8_t c) {
  return nibbles4(0b0001, 0b0111, 0b0001 | ((r & 7) << 1), c & 15);
}

uint16_t emit_clr15(void) { return nibbles2(0b1000, 0b1100); }

uint16_t emit_clr(bool r) { return nibbles2(0b1000, 0b0001 | (r << 3)); }

uint16_t emit_clrp(void) { return nibbles2(0b1000, 0b0100); }

uint16_t emit_cmp(void) { return nibbles2(0b1000, 0b0010); }

uint16_t emit_cmpaxh(bool r, bool s) {
  return nibbles2(0b1100 | r, 0b0001 | (s << 3));
}

uint32_t emit_cmpi(bool d, uint16_t i) {
  return ((uint32_t)nibbles4(0, 0b0010 | d, 0b1000, 0) << 16) | i;
}

uint16_t emit_cmpis(bool d, uint8_t i) { return nibbles2(0, 0b0110 | d) | i; }

uint16_t emit_dar(uint8_t d) { return 0b100 | (d & 3); }

uint16_t emit_dec(bool d) { return nibbles2(0b0111, 0b1010 | d); }

uint16_t emit_decm(bool d) { return nibbles2(0b0111, 0b1000 | d); }

uint16_t emit_halt(void) { return nibbles4(0, 0, 0b0010, 0b0001); }

uint16_t emit_iar(uint8_t d) { return nibbles4(0, 0, 0, 0b1000 | (d & 3)); }

uint16_t emit_ifcc(uint8_t c) { return nibbles4(0, 0b0010, 0b0111, c & 15); }

uint16_t emit_ilrr(bool d, uint8_t s) {
  return nibbles4(0, 0b0010 | d, 0b0001, s & 3);
}

uint16_t emit_ilrrd(bool d, uint8_t s) {
  return nibbles4(0, 0b0010 | d, 0b0001, 0b0100 | (s & 3));
}

uint16_t emit_ilrri(bool d, uint8_t s) {
  return nibbles4(0, 0b0010 | d, 0b0001, 0b1000 | (s & 3));
}

uint16_t emit_ilrrn(bool d, uint8_t s) {
  return nibbles4(0, 0b0010 | d, 0b0001, 0b1100 | (s & 3));
}

uint16_t emit_inc(bool d) { return nibbles2(0b0111, 0b0110 | d); }

uint16_t emit_incm(bool d) { return nibbles2(0b0111, 0b0100 | d); }

uint32_t emit_jmp(uint16_t a) {
  return ((uint32_t)nibbles4(0, 0b0010, 0b1001, 0b1111) << 16) | a;
}

uint32_t emit_jcc(uint8_t c, uint16_t a) {
  return ((uint32_t)nibbles4(0, 0b0010, 0b1001, c & 15) << 16) | a;
}

uint16_t emit_jmpr(uint8_t r) {
  return nibbles4(0b0001, 0b0111, (r & 7) << 1, 0b1111);
}

uint16_t emit_jrcc(uint8_t r, uint8_t c) {
  return nibbles4(0b0001, 0b0111, (r & 7) << 1, c & 15);
}

uint16_t emit_loop(uint8_t r) {
  return nibbles4(0, 0, 0b0100 | ((r >> 4) & 1), r & 15);
}

uint16_t emit_loopi(uint8_t i) { return nibbles2(0b0001, 0) | i; }

uint32_t emit_lr(uint16_t d, uint16_t m) {
  return ((uint32_t)nibbles4(0, 0, 0b1100 | ((d >> 4) & 1), d & 15) << 16) | m;
}

uint32_t emit_lri(uint16_t d, uint16_t i) {
  return ((uint32_t)nibbles4(0, 0, 0b1000 | ((d >> 4) & 1), d & 15) << 16) | i;
}

uint16_t emit_lris(uint16_t d, uint8_t i) {
  return nibbles2(0, 0b1000 | (d & 7)) | i;
}

uint16_t emit_lrr(uint16_t s, uint16_t d) {
  return nibbles4(0b0001, 0b1000, ((s & 3) << 1) | ((d >> 4) & 1), d & 15);
}

uint16_t emit_lrrd(uint16_t s, uint16_t d) {
  return nibbles4(0b0001, 0b1000, 0b1000 | ((s & 3) << 1) | ((d >> 4) & 1),
                  d & 15);
}

uint16_t emit_lrri(uint16_t s, uint16_t d) {
  return nibbles4(0b0001, 0b1001, ((s & 3) << 1) | ((d >> 4) & 1), d & 15);
}

uint16_t emit_lrrn(uint16_t s, uint16_t d) {
  return nibbles4(0b0001, 0b1001, 0b1000 | ((s & 3) << 1) | ((d >> 4) & 1),
                  d & 15);
}

uint16_t emit_lrs(uint16_t d, uint8_t m) { return nibbles2(0b0010, d & 7) | m; }

uint16_t emit_lsl(bool r, uint16_t i) {
  return nibbles2(0b0001, 0b0100 | r) | (i & 0x3f);
}

uint16_t emit_lsl16(bool r) { return nibbles2(0b1111, r); }

uint16_t emit_lsr(bool r, uint8_t i) {
  return nibbles2(0b0001, 0b0100 | r) | (i & 0x7f);
}

uint16_t emit_lsrn(void) { return nibbles4(0, 0b0010, 0b1100, 0b1010); }

uint16_t emit_lsrnr(bool d) { return nibbles4(0b0011, 0b1100 | d, 0b1000, 0); }

uint16_t emit_lsrnrx(bool s, bool d) {
  return nibbles4(0b0011, 0b0100 | (s << 1) | d, 0b1000, 0);
}

uint16_t emit_lsr16(bool r) { return nibbles2(0b1111, 0b0100 | r); }

uint16_t emit_m0(void) { return nibbles2(0b1000, 0b1011); }

uint16_t emit_m2(void) { return nibbles2(0b1000, 0b1010); }

uint16_t emit_madd(bool s) { return nibbles2(0b1111, 0b0010 | s); }

uint16_t emit_maddc(bool s, bool t) {
  return nibbles2(0b1110, 0b1000 | (s << 1) | t);
}

uint16_t emit_maddx(bool s, bool t) { return nibbles2(0b1110, (s << 1) | t); }

uint16_t emit_mov(bool d) { return nibbles2(0b01110, 0b1100 | d); }

uint16_t emit_movax(bool s, bool d) {
  return nibbles2(0b0110, 0b1000 | (s << 1) | d);
}

uint16_t emit_movnp(bool d) { return nibbles2(0b0111, 0b1110 | d); }

uint16_t emit_movp(bool d) { return nibbles2(0b0110, 0b1110 | d); }

uint16_t emit_movpz(bool d) { return nibbles2(0b1111, 0b1110 | d); }

uint16_t emit_movr(uint16_t s, bool d) {
  return nibbles2(0b1110, ((s & 3) << 1) | d);
}

uint16_t emit_mrr(uint16_t d, uint16_t s) {
  return nibbles4(0b0001, 0b1100 | ((d >> 3) & 3),
                  ((d & 7) << 1) | ((s >> 4) & 1), s & 15);
}

uint16_t emit_msub(bool s) { return nibbles2(0b1111, 0b0110 | s); }

uint16_t emit_msubc(bool s, bool t) {
  return nibbles2(0b1110, 0b1100 | (s << 1) | t);
}

uint16_t emit_msubx(bool s, bool t) {
  return nibbles2(0b1110, 0b0100 | (s << 1) | t);
}

uint16_t emit_mul(bool s) { return nibbles2(0b1001, s << 3); }

uint16_t emit_mulac(bool s, bool r) {
  return nibbles2(0b1001, 0b0100 | (s << 3) | r);
}

uint16_t emit_mulaxh(void) { return nibbles2(0b1000, 0b0011); }

uint16_t emit_mulc(bool s, bool t) { return nibbles2(0b1100 | s, t << 3); }

uint16_t emit_mulcac(bool s, bool t, bool r) {
  return nibbles2(0b1100 | s, 0b0100 | (t << 3) | r);
}

uint16_t emit_mulcmv(bool s, bool t, bool r) {
  return nibbles2(0b1100 | s, 0b0110 | (t << 3) | r);
}

uint16_t emit_mulcmvz(bool s, bool t, bool r) {
  return nibbles2(0b1100 | s, 0b0010 | (t << 3) | r);
}

uint16_t emit_mulmv(bool s, bool r) {
  return nibbles2(0b1001, 0b0110 | (s << 3) | r);
}

uint16_t emit_mulmvz(bool s, bool r) {
  return nibbles2(0b1001, 0b0010 | (s << 3) | r);
}

uint16_t emit_mulx(bool s, bool t) { return nibbles2(0b1010 | s, t << 3); }

uint16_t emit_mulxac(bool s, bool t, bool r) {
  return nibbles2(0b1010 | s, 0b0100 | (t << 3) | r);
}

uint16_t emit_mulxmv(bool s, bool t, bool r) {
  return nibbles2(0b1010 | s, 0b0110 | (t << 3) | r);
}

uint16_t emit_mulxmvz(bool s, bool t, bool r) {
  return nibbles2(0b1010 | s, 0b0010 | (t << 3) | r);
}

uint16_t emit_neg(bool d) { return nibbles2(0b0111, 0b1100 | d); }

uint16_t emit_not(bool d) { return nibbles4(0b0011, 0b0010 | d, 0b1000, 0); }

uint16_t emit_nop(void) { return 0; }

uint16_t emit_nx(void) { return nibbles2(0b1000, 0); }

uint16_t emit_orc(bool d) { return nibbles2(0b0011, 0b1110 | d); }

uint32_t emit_ori(bool d, uint16_t i) {
  return ((uint32_t)nibbles4(0, 0b0010 | d, 0b0110, 0) << 16) | i;
}

uint16_t emit_orr(bool s, bool d) {
  return nibbles2(0b0011, 0b1000 | (s << 1) | d);
}

uint16_t emit_ret(void) { return nibbles4(0, 0b0010, 0b1101, 0b1111); }

uint16_t emit_retcc(uint8_t c) { return nibbles4(0, 0b0010, 0b1101, c & 15); }

uint16_t emit_rti(void) { return nibbles4(0, 0b0010, 0b1111, 0b1111); }

uint16_t emit_rticc(uint8_t c) { return nibbles4(0, 0b0010, 0b1111, c & 15); }

uint16_t emit_sbclr(uint8_t i) { return nibbles4(0b0001, 0b0010, 0, i & 7); }

uint16_t emit_sbset(uint8_t i) { return nibbles4(0b0001, 0b0011, 0, i & 7); }

uint16_t emit_set15(void) { return nibbles2(0b1000, 0b1101); }

uint16_t emit_set16(void) { return nibbles2(0b1000, 0b1110); }

uint16_t emit_set40(void) { return nibbles2(0b1000, 0b1111); }

uint32_t emit_si(uint8_t m, uint16_t i) {
  return ((uint32_t)(nibbles2(0b0001, 0b0110) | m) << 16) | i;
}

uint32_t emit_sr(uint16_t s, uint16_t m) {
  return ((uint32_t)(nibbles4(0, 0, 0b1110 | ((s >> 4) & 1), s & 15)) << 16) |
         m;
}

uint16_t emit_srr(uint16_t d, uint16_t s) {
  return nibbles4(0b0001, 0b1010, ((d & 3) << 1) | ((s >> 4) & 1), s & 15);
}

uint16_t emit_srrd(uint16_t d, uint16_t s) {
  return nibbles4(0b0001, 0b1010, 0b1000 | ((d & 3) << 1) | ((s >> 4) & 1),
                  s & 15);
}

uint16_t emit_srri(uint16_t d, uint16_t s) {
  return nibbles4(0b0001, 0b1011, ((d & 3) << 1) | ((s >> 4) & 1), s & 15);
}

uint16_t emit_srrn(uint16_t d, uint16_t s) {
  return nibbles4(0b0001, 0b1011, 0b1000 | ((d & 3) << 1) | ((s >> 4) & 1),
                  s & 15);
}

uint16_t emit_srs(uint16_t s, uint8_t m) {
  return nibbles2(0b0010, 0b1100 | (s & 3)) | m;
}

uint16_t emit_srsh(bool s, uint8_t m) {
  return nibbles2(0b0010, 0b1000 | s) | m;
}

uint16_t emit_sub(bool d) { return nibbles2(0b0101, 0b1100 | d); }

uint16_t emit_subarn(uint16_t d) { return 0b1100 | (d & 3); }

uint16_t emit_subax(bool s, bool d) {
  return nibbles2(0b0101, 0b1000 | (s << 1) | d);
}

uint16_t emit_subp(bool d) { return nibbles2(0b0101, 0b1110 | d); }

uint16_t emit_subr(uint16_t s, bool d) {
  return nibbles2(0b0101, ((s & 3) << 1) | d);
}

uint16_t emit_tst(bool r) { return nibbles2(0b1011, 0b0001 | (r << 3)); }

uint16_t emit_tstaxh(bool r) { return nibbles2(0b1000, 0b0110 | r); }

uint16_t emit_tstprod(void) { return nibbles2(0b1000, 0b0101); }

uint16_t emit_xorc(bool d) { return nibbles4(0b0011, d, 0b1000, 0); }

uint32_t emit_xori(bool d, uint16_t i) {
  return ((uint32_t)nibbles4(0, 0b0010 | d, 0b0010, 0) << 16) | i;
}

uint16_t emit_xorr(bool s, bool d) { return nibbles2(0b0011, (s << 1) | d); }
