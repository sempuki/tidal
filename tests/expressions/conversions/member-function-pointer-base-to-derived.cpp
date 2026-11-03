// tidal-rules: CONV-15
// tidal-expect: accept
// tidal-note: A pointer to a member function of the base, used as a pointer to a member function of the derived class. The call runs the base's function.
struct B { int v = 3; constexpr int get() const { return v; } };
struct D : B { int w = 9; };
constexpr int (D::* pf)() const = &B::get;
static_assert((D{}.*pf)() == 3);
