// tidal-rules: OVL-20
// tidal-expect: accept
// tidal-expect-gcc-14: reject
// tidal-expect-clang-18: reject
// tidal-expect-edg: reject
// tidal-error-gcc-14: unable to deduce .const auto. from
// tidal-error-clang-18: address of overloaded function .f. is ambiguous
// tidal-error-edg: cannot deduce "auto" type
// tidal-note: Q-0160. Both constraints hold for char. Tiny subsumes Small, so the less constrained function leaves the set. GCC 14, Clang 18, and EDG find no single function.
template <class T> concept Small = sizeof(T) <= 8;
template <class T> concept Tiny = Small<T> && sizeof(T) == 1;
template <class T> struct Size {
  static constexpr int f() requires Small<T> { return 1; }
  static constexpr int f() requires Tiny<T> { return 2; }
};
constexpr auto p = &Size<char>::f;
static_assert(p() == 2);
