// tidal-rules: PACK-20
// tidal-expect: reject
// tidal-error-clang: missing parentheses around the size of parameter pack
// tidal-error-gcc: argument must be surrounded by parentheses
// tidal-error-edg: expected a "\("
template <class... T> constexpr auto count = sizeof... T;
