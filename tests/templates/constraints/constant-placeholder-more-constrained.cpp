// tidal-rules: CNS-1 CNS-23
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-expect-gcc-14: reject
// tidal-expect-clang-18: reject
// tidal-error-clang: call to 'pick' is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: The type constraint of N makes the first template more constrained. EDG, GCC 14, and Clang 18 find the call ambiguous (Q-0281).
template <class T> concept Wide = sizeof(T) > 1;
template <Wide auto N> constexpr int pick() { return 1; }
template <auto N> constexpr int pick() { return 2; }
static_assert(pick<5>() == 1);
