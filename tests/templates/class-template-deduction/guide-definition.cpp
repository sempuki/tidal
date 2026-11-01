// tidal-rules: CTAD-12
// tidal-expect: reject
// tidal-error-clang: deduction guide cannot have a function definition
// tidal-error-gcc: must not have a function body
// tidal-error-edg: cannot be defined
template <class T> struct Box { Box(T) {} };
template <class T> Box(T*) -> Box<T> {}
