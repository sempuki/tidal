// tidal-rules: COND-16 COND-3
// tidal-expect: accept
// tidal-note: Wide has conversion functions to int and long. The conversion to int is the best implicit conversion sequence to int, so the result is an int prvalue.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
struct Wide { operator int(); operator long(); };
extern bool c;
Wide w;
int i;
static_assert(same<decltype(c ? w : i), int>);
