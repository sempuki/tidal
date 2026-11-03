// tidal-rules: AUTO-4
// tidal-expect: accept
// tidal-note: A volatile lvalue, a class prvalue, and a const xvalue.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S { int v; };
volatile int vi = 0;
const S cs{};
auto&& a = vi;
auto&& b = S{};
auto&& c = static_cast<const S&&>(cs);
static_assert(same<decltype(a), volatile int&>);
static_assert(same<decltype(b), S&&>);
static_assert(same<decltype(c), const S&&>);
