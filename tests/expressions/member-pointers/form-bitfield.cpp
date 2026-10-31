// tidal-rules: MPTR-9
// tidal-expect: reject
// tidal-error-clang: address of bit-field requested
// tidal-error-gcc: invalid pointer to bit-field
// tidal-error-edg: address of a bit field
struct B { int x : 3; };
auto pm = &B::x;
