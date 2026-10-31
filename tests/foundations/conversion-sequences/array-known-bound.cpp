// tidal-rules: ICS-24
// tidal-expect: accept
// tidal-note: Both arrays get two elements by the identity, so ICS-23 and every other rule tie.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One take(int (&&)[2]);
Two take(int (&&)[]);
static_assert(same<decltype(take({1, 2})), One>);
