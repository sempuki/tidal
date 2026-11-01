// tidal-rules: CPT-29
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: default arguments not allowed for parameters of a requires expression
// tidal-error-edg: default argument is not allowed
// tidal-note: GCC accepts a default argument here (Q-0259).
template <class T> concept Copies = requires (T t = T()) { t; };
