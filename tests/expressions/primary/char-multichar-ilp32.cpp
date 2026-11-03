// tidal-rules: PRIM-6
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: The PRIM-6 table on the -m32 target.
static_assert('ab' == 0x6162);
static_assert('abcd' == 0x61626364);
static_assert(static_cast<char>(-1) < 0);
