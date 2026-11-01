// tidal-rules: PRIM-29
// tidal-expect: reject
// tidal-error-clang: empty expansion
// tidal-error-gcc: fold of empty expansion
// tidal-error-edg: empty expansion
template <int... V> constexpr int sum = (V + ...);
int total = sum<>;
