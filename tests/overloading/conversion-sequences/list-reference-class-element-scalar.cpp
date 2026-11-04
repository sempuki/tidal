// tidal-rules: ICS-20
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: static assertion failed
// tidal-note: Q-0157. Source is not int, so REF-14 makes a temporary int from {s}, and int& cannot bind the temporary. edit(long) is the only viable candidate. GCC binds int& through operator int& and chooses edit(int&).
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Source { operator int&(); };
Source s;
One edit(int&);
Two edit(long);
static_assert(same<decltype(edit({s})), Two>);
