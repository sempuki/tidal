// tidal-rules: VIRT-15
// tidal-expect: reject
// tidal-error-clang: has a different return type
// tidal-error-gcc: conflicting return type specified
// tidal-error-edg: not identical to nor covariant with
struct B { virtual int f(); };
struct D : B { long f(); };
