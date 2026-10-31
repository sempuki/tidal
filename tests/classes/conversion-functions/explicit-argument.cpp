// tidal-rules: CVF-3
// tidal-expect: reject
// tidal-error-clang: no matching function for call to .take
// tidal-error-gcc: cannot convert .S. to .int
// tidal-error-edg: no suitable conversion function
struct S { explicit operator int() const { return 1; } };
void take(int);
void f() { take(S{}); }
