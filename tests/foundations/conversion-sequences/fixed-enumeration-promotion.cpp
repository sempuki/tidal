// tidal-rules: ICS-39
// tidal-expect: accept
// tidal-note: The enumeration promotes to short and to int, so each beats the conversion to long by rank. ICS-34 does not decide, because only one target is a promotion.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
enum Level : short { low };
enum Code : int { none };
One take(short);
Two take(long);
static_assert(same<decltype(take(low)), One>);
One wide(int);
Two wide(long);
static_assert(same<decltype(wide(low)), One>);
One code(int);
Two code(long);
static_assert(same<decltype(code(none)), One>);
