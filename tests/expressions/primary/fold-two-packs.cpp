// tidal-rules: PRIM-29
// tidal-expect: reject
// tidal-error-clang: unexpanded parameter packs in both operands
// tidal-error-gcc: both arguments in binary fold have unexpanded parameter packs
// tidal-error-edg: binary fold expression cannot apply to two
template <int... A> struct Outer {
  template <int... B> static constexpr int both = (A + ... + B);
};
int total = Outer<1>::both<2>;
