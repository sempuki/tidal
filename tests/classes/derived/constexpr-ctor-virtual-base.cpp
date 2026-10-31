// tidal-rules: DER-6
// tidal-expect: reject
struct A {};
struct D : virtual A { constexpr D() {} };
