// tidal-rules: UNI-2
// tidal-expect: reject
// tidal-error-clang: union member .r. has reference type
// tidal-error-gcc: in a union may not have reference type
// tidal-error-edg: member with reference type is not allowed in a union
int g;
union U { int& r; };
