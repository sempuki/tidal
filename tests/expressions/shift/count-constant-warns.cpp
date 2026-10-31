// tidal-rules: SHF-9
// tidal-expect: reject
// tidal-error-clang: shift count >= width of type
// tidal-error-gcc: shift count >= width of type
// tidal-flags: -Werror
int f() { return 1 << 33; }
