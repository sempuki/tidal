// tidal-rules: UNI-8
// tidal-expect: reject
// tidal-error-clang: initializing multiple members of union
// tidal-error-gcc: initializations for multiple members
// tidal-error-edg: only one member of a union may be specified
union U { int i; float f; U() : i(1), f(2.0f) {} };
