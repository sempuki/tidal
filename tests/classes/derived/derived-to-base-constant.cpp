// tidal-rules: DER-5
// tidal-expect: accept
struct A { int a = 1; };
struct B { int b = 2; };
struct D : A, B {};
constexpr D d{};
constexpr const B* p = &d;
static_assert(p->b == 2);
