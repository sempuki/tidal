// tidal-rules: EXS-3
// tidal-expect: reject
struct B { virtual void f() noexcept; };
struct D : B { void f() override; };
