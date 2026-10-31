// tidal-rules: DER-1
// tidal-expect: reject
// tidal-error-clang: is marked .final
// tidal-error-gcc: cannot derive from .final. base
// tidal-error-edg: final. class type cannot be used as a base class
struct B final {};
struct D : B {};
