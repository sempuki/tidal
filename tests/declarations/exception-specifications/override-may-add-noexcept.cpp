// tidal-rules: EXS-3
// tidal-expect: accept
struct B { virtual void f(); };
struct D : B { void f() noexcept override; };
