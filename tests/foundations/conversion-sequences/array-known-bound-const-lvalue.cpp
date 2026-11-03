// tidal-rules: ICS-24
// tidal-expect: accept
// tidal-note: Const lvalue references to arrays of long. Both arrays get three elements by the identity, so ICS-23 and every other rule tie.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One take(const long (&)[3]);
Two take(const long (&)[]);
static_assert(same<decltype(take({1L, 2L, 3L})), One>);
