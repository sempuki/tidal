// tidal-rules: CVF-2
// tidal-expect: reject
struct A { operator int() const { return 1; } };
struct B { B(int) {} };
void take(B);
void f() { take(A{}); }
