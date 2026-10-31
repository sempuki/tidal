// tidal-rules: VIRT-15
// tidal-expect: reject
// tidal-error-clang: is not covariant with the return type|has a different return type
// tidal-error-gcc: invalid covariant return type
// tidal-error-edg: not identical to nor covariant with
struct B { virtual B* get(); };
struct D : B { const D* get() override; };
