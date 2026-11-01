// tidal-rules: ICS-28 ICS-34
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-expect-edg: reject
// tidal-error-gcc: static assertion failed
// tidal-error-edg: static assertion failed
// tidal-note: Q-0152. Both sequences are promotions that bind to a temporary. ICS-34 prefers short, and ICS-28 prefers the rvalue reference to int. ICS-34 comes first. GCC and EDG apply ICS-28 first.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
enum Level : short { low };
One take(const short&);
Two take(int&&);
static_assert(same<decltype(take(low)), One>);
