// tidal-rules: DER-7
// tidal-expect: accept
class B { protected: ~B() {} };
struct D : B {};
void f() { D d; }
