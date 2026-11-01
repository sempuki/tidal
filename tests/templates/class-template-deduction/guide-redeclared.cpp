// tidal-rules: CTAD-12
// tidal-expect: reject
// tidal-error-clang: redeclaration of deduction guide
// tidal-error-gcc: redefinition of
// tidal-error-edg: redeclaration of deduction guide
template <class T> struct Box { Box(T) {} };
template <class T> Box(T*) -> Box<T>; template <class T> Box(T*) -> Box<T>;
