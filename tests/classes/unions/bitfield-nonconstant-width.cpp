// tidal-rules: BF-2
// tidal-expect: reject
// tidal-error-clang: not an integral constant expression
// tidal-error-gcc: not usable in a constant expression
// tidal-error-edg: must have a constant value
int w = 3;
struct S { int x : w; };
