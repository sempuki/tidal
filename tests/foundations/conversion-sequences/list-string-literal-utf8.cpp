// tidal-rules: ICS-38
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: static assertion failed
// tidal-note: Q-0155. A UTF-8 literal can initialize a char array (LIST-6), so the braced literal has the identity. GCC gives it no sequence and chooses Utf8.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Utf8 { Utf8(const char8_t*); };
One take(const char (&)[3]);
Two take(Utf8);
static_assert(same<decltype(take({u8"ab"})), One>);
