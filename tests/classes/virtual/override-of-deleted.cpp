// tidal-rules: VIRT-19
// tidal-expect: reject
// tidal-error-clang: non-deleted function .f. cannot override a deleted function
// tidal-error-gcc: non-deleted function .* overriding deleted function
// tidal-error-edg: nondeleted function overrides deleted function
struct B { virtual void f() = delete; };
struct D : B { void f() override; };
