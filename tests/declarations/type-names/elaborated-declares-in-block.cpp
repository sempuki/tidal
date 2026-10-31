// tidal-rules: TYN-10
// tidal-expect: reject
// tidal-error-clang: unknown type name .Local
// tidal-error-gcc: .Local. does not name a type
// tidal-error-edg: identifier .Local. is undefined
void f() { struct Local* p = nullptr; (void)p; }
Local* q;
