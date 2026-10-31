// tidal-rules: ICS-28 ICS-30
// tidal-expect: accept
// tidal-note: ICS-30 prefers const int& and ICS-28 prefers the rvalue reference. ICS-28 comes first.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One take(const volatile int&&);
Two take(const int&);
static_assert(same<decltype(take(1)), One>);
struct Item {};
One put(const volatile Item&&);
Two put(const Item&);
static_assert(same<decltype(put(Item{})), One>);
