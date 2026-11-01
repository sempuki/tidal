// tidal-rules: ICS-28 ICS-31
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: static assertion failed
// tidal-note: Q-0152. Both references bind to a temporary pointer. ICS-31 prefers the one that adds fewer cv-qualifiers, and ICS-28 prefers the rvalue reference. ICS-28 comes first. GCC applies ICS-31 first.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
int n = 0;
One take(const int* const&);
Two take(const volatile int*&&);
static_assert(same<decltype(take(&n)), Two>);
