// tidal-rules: ICS-39
// tidal-expect: accept
// tidal-note: The enumeration with underlying type unsigned char promotes to unsigned char and to int. Each promotion beats a conversion to another type by rank. ICS-34 does not decide, because only one target is a promotion.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
enum Flag : unsigned char { off };
One narrow(unsigned char);
Two narrow(unsigned int);
static_assert(same<decltype(narrow(off)), One>);
One wide(int);
Two wide(double);
static_assert(same<decltype(wide(off)), One>);
