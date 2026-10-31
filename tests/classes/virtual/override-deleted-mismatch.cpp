// tidal-rules: VIRT-19
// tidal-expect: reject
// tidal-error-clang: deleted function .f. cannot override a non-deleted function
// tidal-error-gcc: deleted function .* overriding non-deleted function
// tidal-error-edg: deleted function overrides nondeleted function
struct B { virtual void f(); };
struct D : B { void f() override = delete; };
