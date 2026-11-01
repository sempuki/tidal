// tidal-rules: TARG-17
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-expect-clang-18: reject
// tidal-error-edg: is not compatible with template template parameter
// tidal-error-clang-18: template template argument has different template parameters
// tidal-note: The argument template's parameter is long, and the head's parameter is int. Clang 22 and GCC accept, Clang 18 and EDG reject (Q-0201).
template <long V> struct Long { static constexpr long value = V; };
template <template <int> class C> struct Use { static constexpr long value = C<3>::value; };
static_assert(Use<Long>::value == 3);
