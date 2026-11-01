// tidal-rules: OVL-20 ORD-13
// tidal-expect: reject
// tidal-expect-clang-18: accept
// tidal-error-clang: incompatible initializer of type
// tidal-error-gcc: unable to deduce
// tidal-error-edg: cannot deduce "auto" type
// tidal-note: Q-0142. Both constraints hold for char, and Tiny subsumes Small. The two functions have different parameter lists, so the conditions of ORD-13 do not hold and the constraints do not remove either function. Two functions remain. Clang 18 keeps the more constrained function.
template <class T> concept Small = sizeof(T) <= 8;
template <class T> concept Tiny = Small<T> && sizeof(T) == 1;
template <class T> struct Size {
  static constexpr int f() requires Small<T> { return 1; }
  static constexpr int f(int) requires Tiny<T> { return 2; }
};
constexpr auto p = &Size<char>::f;
