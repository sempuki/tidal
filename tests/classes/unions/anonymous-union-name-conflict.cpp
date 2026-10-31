// tidal-rules: UNI-9
// tidal-expect: reject
// tidal-error-clang: member of anonymous union redeclares .i
// tidal-error-gcc: conflicts with a previous declaration
// tidal-error-edg: has already been declared in the current scope
struct S { int i; union { int i; float f; }; };
