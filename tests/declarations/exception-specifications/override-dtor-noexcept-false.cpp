// tidal-rules: EXS-3
// tidal-expect: reject
struct B { virtual ~B() noexcept; };
struct D : B { ~D() noexcept(false) override; };
