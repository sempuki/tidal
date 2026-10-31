// tidal-rules: DER-14
// tidal-expect: reject
class B { ~B() {} };
struct D : B {};
void f() { D d; }
