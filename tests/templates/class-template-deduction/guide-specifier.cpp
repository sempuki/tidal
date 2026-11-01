// tidal-rules: CTAD-12
// tidal-expect: reject
// tidal-error-clang: deduction guide cannot be declared 'constexpr'
// tidal-error-gcc: in declaration of deduction guide
// tidal-error-edg: invalid specifier for deduction guide
template <class T> struct Box { Box(T) {} };
template <class T> constexpr Box(T*) -> Box<T>;
