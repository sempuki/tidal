// tidal-rules: CTAD-28 CTAD-14 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide|requires template arguments
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor
// tidal-note: Twin names Wide, whose constraint needs sizeof(T) > 1. The guides of Twin come through Wide, and the constraint of Wide still applies, so char finds no viable guide. Clang 18 has no deduction through an alias (Q-0240) and rejects too.
template <class T> concept Big = sizeof(T) > 1;
template <class A, class B> struct Pair { constexpr Pair(A, B) {} };
template <class T> requires Big<T> using Wide = Pair<T, T>;
template <class T> using Twin = Wide<T>;
Twin p('a', 'b');
