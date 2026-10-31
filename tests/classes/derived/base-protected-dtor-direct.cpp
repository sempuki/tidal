// tidal-rules: DER-7
// tidal-expect: reject
class B { protected: ~B() {} };
void f() { B b; }
