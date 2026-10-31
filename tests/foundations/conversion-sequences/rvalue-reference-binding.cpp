// tidal-rules: ICS-28
// tidal-expect: accept
// tidal-note: Both bind directly with the same cv-qualifiers, so rank and ICS-30 tie. Only ICS-28 decides.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One take(const int&&);
Two take(const int&);
static_assert(same<decltype(take(1)), One>);
struct Item {};
One put(const Item&&);
Two put(const Item&);
static_assert(same<decltype(put(Item{})), One>);
