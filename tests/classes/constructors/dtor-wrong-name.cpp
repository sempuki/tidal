// tidal-rules: DTOR-1
// tidal-expect: reject
// tidal-error-clang: in destructor name
// tidal-error-gcc: expected class-name
// tidal-error-edg: invalid destructor declaration
struct S { ~T(); };
