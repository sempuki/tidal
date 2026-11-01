// tidal-rules: TPL-14
// tidal-expect: reject
// tidal-error-clang: template parameter missing a default argument
// tidal-error-gcc: no default argument for
// tidal-error-edg: default argument not at end of parameter list
template <int N = 0, int M> constexpr int sum = N + M;
