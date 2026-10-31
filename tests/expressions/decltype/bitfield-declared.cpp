// tidal-rules: DTY-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct B { unsigned short x : 3; };
B b;
static_assert(same<decltype(b.x), unsigned short>);
