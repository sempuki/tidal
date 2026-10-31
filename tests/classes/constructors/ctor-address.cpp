// tidal-rules: CTOR-1 NAM-34
// tidal-expect: reject
// tidal-error-clang: is a constructor name rather than a type
// tidal-error-gcc: taking address of constructor
// tidal-error-edg: may not have its address taken
struct S { S(); };
auto p = &S::S;
