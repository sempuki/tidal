// tidal-rules: DER-14
// tidal-expect: reject
class B { protected: ~B() {} };
void f() { B b; }
