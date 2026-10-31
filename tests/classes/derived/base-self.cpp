// tidal-rules: DER-1
// tidal-expect: reject
// tidal-error-clang: base class has incomplete type
// tidal-error-gcc: invalid use of incomplete type
// tidal-error-edg: incomplete type .* is not allowed
struct D : D {};
