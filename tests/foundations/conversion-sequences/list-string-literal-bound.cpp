// tidal-rules: ICS-38 ICS-46
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: ambiguous
// tidal-note: Q-0155, Q-0188. Both references bind directly to the literal, and ICS-46 prefers the known bound. Clang finds a tie.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One take(const char (&)[]);
Two take(const char (&)[3]);
static_assert(same<decltype(take({"ab"})), Two>);
