// tidal-rules: DER-1
// tidal-expect: reject
// tidal-error-clang: unions cannot be base classes
// tidal-error-gcc: fails to be a struct or class type
// tidal-error-edg: cannot not derive from a union
union U { int i; };
struct D : U {};
