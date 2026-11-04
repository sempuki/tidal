// tidal-rules: ICS-33
// tidal-expect: accept
// tidal-note: Both conversions have conversion rank. Neither target is bool, and void is not a class, so ICS-36 does not apply.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Base {};
struct Derived : Base {};
Derived* d = nullptr;
One take(Base*);
Two take(void*);
static_assert(same<decltype(take(d)), One>);
