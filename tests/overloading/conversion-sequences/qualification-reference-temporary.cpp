// tidal-rules: ICS-31
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: Q-0153. The reference binds a temporary const int*, and the value parameter is a const volatile int*. ICS-31 prefers the one that adds fewer cv-qualifiers. EDG finds a tie.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
int n = 0;
One take(const int* const&);
Two take(const volatile int*);
static_assert(same<decltype(take(&n)), One>);
