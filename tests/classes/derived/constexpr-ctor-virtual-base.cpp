// tidal-rules: DER-12
// tidal-expect: reject
// tidal-error-clang: not allowed in struct with virtual base class
// tidal-error-gcc: has virtual base classes
// tidal-error-edg: class with virtual bases cannot be constexpr
struct A {};
struct D : virtual A { constexpr D() {} };
