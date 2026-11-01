// tidal-rules: CPT-29
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: varargs not allowed in requires expression
// tidal-error-edg: ellipsis parameter is not permitted
// tidal-note: GCC accepts an ellipsis here (Q-0259).
template <class T> concept Copies = requires (T t, ...) { t; };
