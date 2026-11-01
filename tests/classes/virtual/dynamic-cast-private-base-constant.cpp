// tidal-rules: VIRT-10
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Q-0212. EDG 7.0 lets a cast from a private base succeed during constant evaluation. Clang and GCC give a null pointer, as at run time.
struct A { virtual constexpr ~A() {} };
struct B { virtual constexpr ~B() {} };
struct D : private A, public B { constexpr A* hidden() { return this; } };
constexpr bool down_fails() { D d; return dynamic_cast<D*>(d.hidden()) == nullptr; }
constexpr bool cross_fails() { D d; return dynamic_cast<B*>(d.hidden()) == nullptr; }
static_assert(down_fails());
static_assert(cross_fails());
