// tidal-rules: OVL-19
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-expect-gcc-14: reject
// tidal-expect-clang-18: reject
// tidal-error-gcc-14: is ambiguous
// tidal-error-clang-18: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: Q-0142. Both members have the type int(), and the second is more constrained. EDG, GCC 14, and Clang 18 find the set ambiguous.
template <class T> concept Small = sizeof(T) <= 8;
template <class T> concept SmallWhole = Small<T> && (T(1) / 2 == 0);
template <class T> struct Box {
  static constexpr int pick() requires Small<T> { return 1; }
  static constexpr int pick() requires SmallWhole<T> { return 2; }
};
constexpr int (*p)() = &Box<int>::pick;
static_assert(p() == 2);
