// tidal-rules: CPT-10
// tidal-expect: reject
// tidal-error-clang: parentheses are required around this expression
// tidal-error-gcc: must be enclosed in parentheses
// tidal-error-edg: cannot appear at the top level
template <class T> constexpr bool big() { return sizeof(T) > 1; }
template <class T> requires big<T>() void use(T);
