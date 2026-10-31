// tidal-rules: THR-3
// tidal-expect: reject
class D { ~D() {} public: D() {} };
void f() { throw D(); }
