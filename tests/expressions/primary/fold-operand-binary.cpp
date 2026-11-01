// tidal-rules: PRIM-29
// tidal-expect: reject
// tidal-error-clang: not permitted as operand of fold expression
// tidal-error-gcc: binary expression in operand of fold-expression
// tidal-expect-edg: accept
// tidal-note: Q-0194
template <int... V> constexpr int scaled = (2 * V + ...);
int total = scaled<1, 2>;
