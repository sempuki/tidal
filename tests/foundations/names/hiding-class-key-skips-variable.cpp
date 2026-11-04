// tidal-rules: NAM-12
// tidal-expect: accept
// tidal-note: After a class key, lookup considers only types. The local variable S does not hide the global class S there.
struct S { int x; };
void f() { int S = 0; struct S s; (void)S; (void)s; }
