// tidal-rules: VIRT-10
// tidal-expect: accept
struct B { constexpr virtual ~B() {} };
struct D : B { int v = 1; };
struct E : B {};
constexpr D d;
constexpr E e;
static_assert(dynamic_cast<const D*>(static_cast<const B*>(&d)) == &d);
static_assert(dynamic_cast<const D*>(static_cast<const B*>(&e)) == nullptr);
