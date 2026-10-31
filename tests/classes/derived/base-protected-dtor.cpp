// tidal-rules: DER-14
// tidal-expect: accept
class B { protected: ~B() {} };
struct D : B {};
void f() { D d; }
