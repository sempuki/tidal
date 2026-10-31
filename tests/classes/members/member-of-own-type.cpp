// tidal-rules: CLS-1
// tidal-expect: reject
// tidal-error-clang: field has incomplete type
// tidal-error-gcc: has incomplete type
// tidal-error-edg: incomplete type .* is not allowed
struct S { S inner; };
