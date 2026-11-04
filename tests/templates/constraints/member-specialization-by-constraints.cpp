// tidal-rules: CNS-37
// tidal-expect: accept
// tidal-note: For Box<int>, the first member's clause is a requires expression that becomes false. The clause is still valid, and it is not equivalent to the specialization's clause, so the specialization matches the second member.
template <class T> struct Box {
  template <class U> static constexpr int f(U) requires requires { typename T::type; } { return 1; }
  template <class U> static constexpr int f(U) requires (sizeof(T) > 1) { return 2; }
};
template <> template <class U> constexpr int Box<int>::f(U) requires (sizeof(int) > 1) { return 3; }
static_assert(Box<int>::f(1) == 3);
static_assert(Box<long>::f(1) == 2);
