// tidal-rules: VIRT-10
// tidal-expect: reject
struct B { virtual ~B() {} };
struct D : B {};
D f(B& b) { return dynamic_cast<D>(b); }
