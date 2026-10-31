// tidal-rules: DER-12
// tidal-expect: reject
// tidal-error-clang: non-literal type
// tidal-error-gcc: virtual base classes
// tidal-error-edg: must have a literal type
struct A {};
struct D : virtual A {};
constexpr D d;
