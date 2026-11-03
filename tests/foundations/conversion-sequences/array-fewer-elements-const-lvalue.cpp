// tidal-rules: ICS-23
// tidal-expect: accept
// tidal-note: Const lvalue references to arrays of double. Each element is the identity, and the other elements of the larger array come from empty braces, so no other rule decides.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One take(const double (&)[1]);
Two take(const double (&)[5]);
static_assert(same<decltype(take({1.0})), One>);
