// tidal-rules: VIRT-15
// tidal-expect: reject
// tidal-error-clang: invalid covariant return for virtual function: .B. is a private base class
// tidal-error-gcc: invalid covariant return type
// tidal-error-edg: not identical to nor covariant with
struct B { virtual B* get(); };
struct X : private B {};
struct D : B { X* get(); };
