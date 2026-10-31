// tidal-rules: EXS-3
// tidal-expect: reject
// tidal-error-clang: overriding function is more lax
// tidal-error-gcc: looser exception specification on overriding
// tidal-error-edg: incompatible with that of overridden function
struct B { virtual void f() noexcept; };
struct D : B { void f() override; };
