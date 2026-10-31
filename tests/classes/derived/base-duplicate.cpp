// tidal-rules: DER-1
// tidal-expect: reject
// tidal-error-clang: specified more than once as a direct base
// tidal-error-gcc: duplicate base type
// tidal-error-edg: duplicate base class name
struct B {};
struct D : B, B {};
