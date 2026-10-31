// tidal-rules: VIRT-18
// tidal-expect: reject
// tidal-error-clang: invalid target type .D. for dynamic_cast
// tidal-error-gcc: target is not pointer or reference
// tidal-error-edg: must be a pointer or reference to a complete class type
struct B { virtual ~B() {} };
struct D : B {};
D f(B& b) { return dynamic_cast<D>(b); }
