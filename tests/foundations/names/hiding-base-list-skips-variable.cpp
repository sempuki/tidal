// tidal-rules: NAM-12
// tidal-expect: accept
// tidal-note: In a list of bases, lookup considers only types. The local variable B does not hide the global class B there.
struct B {};
void f() { int B = 0; (void)B; struct D : B {}; }
