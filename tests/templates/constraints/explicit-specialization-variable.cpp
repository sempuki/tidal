// tidal-rules: CNS-13
// tidal-expect: reject
// tidal-error-clang: constraints not satisfied for variable template
// tidal-error-gcc: does not match any template declaration
// tidal-error-edg: template constraint not satisfied
template <class T> requires (sizeof(T) > 1) constexpr int one = 1;
template <> constexpr int one<char> = 2;
