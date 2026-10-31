// tidal-rules: ICS-2
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-expect-edg: reject
// tidal-error-gcc: call of overloaded .take.* is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: Q-0134. The bit-field of width 3 promotes to int (CONV-1), so int has promotion rank and long conversion rank. GCC and EDG give both conversion rank.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Bits { long long v : 3; };
Bits bits{};
One take(int);
Two take(long);
static_assert(same<decltype(take(bits.v)), One>);
