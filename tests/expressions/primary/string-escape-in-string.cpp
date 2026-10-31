// tidal-rules: PRIM-7
// tidal-expect: accept
static_assert(sizeof("a\x41\n") == 4);
static_assert("\x41"[0] == 'A');
