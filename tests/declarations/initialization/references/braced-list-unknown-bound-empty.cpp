// tidal-rules: REF-11
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: zero size arrays
// tidal-error-edg: empty initializer is invalid for an array
// tidal-note: Q-0149. GCC makes an array of no elements.
int (&&values)[]{};
