// tidal-rules: THR-3
// tidal-expect: reject
class P { P(const P&); public: P() {} };
P global;
void f() { throw global; }
