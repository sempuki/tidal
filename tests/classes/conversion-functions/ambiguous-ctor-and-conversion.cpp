// tidal-rules: CVF-4
// tidal-expect: reject
struct B;
struct A { operator B() const; };
struct B { B() {} B(const A&) {} };
void take(B);
void f() { take(A{}); }
