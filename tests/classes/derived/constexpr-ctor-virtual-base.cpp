// tidal-rules: DER-12
// tidal-expect: reject
struct A {};
struct D : virtual A { constexpr D() {} };
