// tidal-rules: UNI-8
// tidal-expect: reject
// tidal-error-clang: initializing multiple members of union
// tidal-error-gcc: multiple fields in union .* initialized
// tidal-error-edg: union can have at most one field initializer
union U { int i = 1; float f = 2.0f; };
