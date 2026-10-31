// tidal-rules: DER-8
// tidal-expect: reject
class B { B(int) {} public: B() {} friend struct D; };
struct D : B { using B::B; };
D d(1);
