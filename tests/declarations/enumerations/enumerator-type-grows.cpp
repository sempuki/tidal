// tidal-rules: ENUM-13
// tidal-expect: accept
// tidal-note: Q-0114. Clang gives the second enumerator the type long, GCC unsigned int, and EDG an unsigned type of 8 bytes.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
enum Grows { first = 0x7fffffff, second,
  holds = second > first && sizeof(second) >= 4 ? 1 : 0,
  kind = same<decltype(second), long> ? 1 : same<decltype(second), unsigned> ? 2 : sizeof(second) == 8 ? 3 : 0 };
static_assert(holds == 1 && kind != 0);
