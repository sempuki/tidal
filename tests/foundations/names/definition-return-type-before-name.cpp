// tidal-rules: NAM-22
// tidal-expect: reject
// tidal-error-clang: unknown type name .type.
// tidal-error-gcc: type. does not name a type
// tidal-error-edg: identifier .type. is undefined
struct C { using type = int; type get(); };
type C::get() { return 0; }
