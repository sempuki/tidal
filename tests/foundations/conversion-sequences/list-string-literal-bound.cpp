// tidal-rules: ICS-38 ICS-24
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: ambiguous
// tidal-note: Q-0155. Both arrays get three elements, and ICS-24 prefers the known bound. Clang finds a tie.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One take(const char (&)[]);
Two take(const char (&)[3]);
static_assert(same<decltype(take({"ab"})), Two>);
