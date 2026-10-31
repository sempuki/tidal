// tidal-rules: TYN-1
// tidal-expect: reject
// tidal-error-clang: unknown type name .I
// tidal-error-gcc: .I. does not name a type
// tidal-error-edg: identifier .I. is undefined
void f() { using I = int; }
I g;
