// tidal-rules: INL-8
// tidal-expect: reject
// tidal-error-clang: incomplete type .S
// tidal-error-gcc: has incomplete type
// tidal-error-edg: incomplete type .S. is not allowed
struct S { static inline S self{}; };
