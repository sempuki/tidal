// tidal-rules: ICS-23
// tidal-expect: accept
// tidal-note: Both sequences are the identity for each element and the third int comes from empty braces by the identity, so no other rule decides.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One take(int (&&)[2]);
Two take(int (&&)[3]);
static_assert(same<decltype(take({1, 2})), One>);
