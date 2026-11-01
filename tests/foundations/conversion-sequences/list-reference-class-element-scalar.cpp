// tidal-rules: ICS-20
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-expect-edg: reject
// tidal-error-clang: static assertion failed
// tidal-error-edg: static assertion failed
// tidal-note: Q-0157. The referenced type int is not a class, so REF-6 binds the reference to {s} as to s, through operator int&. Both functions then use operator int&, and the third parts decide. Clang and EDG make a temporary, so edit(int&) has no sequence.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Source { operator int&(); };
Source s;
One edit(int&);
Two edit(long);
static_assert(same<decltype(edit({s})), One>);
