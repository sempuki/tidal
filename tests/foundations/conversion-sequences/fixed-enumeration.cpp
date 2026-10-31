// tidal-rules: ICS-34
// tidal-expect: accept
// tidal-note: Both sequences are promotions, so rank ties. Only ICS-34 decides.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
enum Level : short { low };
One take(short);
Two take(int);
static_assert(same<decltype(take(low)), One>);
enum Code : unsigned char { none };
One code(unsigned char);
Two code(int);
static_assert(same<decltype(code(none)), One>);
