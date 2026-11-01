// tidal-rules: PRIM-29
// tidal-expect: reject
// tidal-error-clang: operators in fold expression must be the same
// tidal-error-gcc: mismatched operator in fold-expression
// tidal-error-edg: second operator in binary fold expression
template <int... V> constexpr int mixed = (V + ... * 1);
int total = mixed<1, 2>;
