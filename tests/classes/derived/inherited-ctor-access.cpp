// tidal-rules: DER-20
// tidal-expect: reject
// tidal-error-clang: calling a private constructor
// tidal-error-gcc: private within this context
// tidal-error-edg: is inaccessible
class B { B(int) {} public: B() {} friend struct D; };
struct D : B { using B::B; };
D d(1);
