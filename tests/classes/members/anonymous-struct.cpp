// tidal-rules: CLS-9
// tidal-expect: reject
// tidal-error-clang: anonymous structs are a GNU extension
// tidal-error-gcc: prohibits anonymous structs
// tidal-error-edg: declaration does not declare anything
struct S { struct { int a; }; };
