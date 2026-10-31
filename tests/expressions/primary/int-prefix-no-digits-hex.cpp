// tidal-rules: PRIM-16
// tidal-expect: reject
// tidal-error-clang: invalid suffix .x. on integer constant
// tidal-error-gcc: numeric literal operator .operator.{2}x.
// tidal-error-edg: invalid hexadecimal number
// tidal-note: Gap review 2026-10-31.
int x = 0x;
