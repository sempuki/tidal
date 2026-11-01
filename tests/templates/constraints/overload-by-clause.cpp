// tidal-rules: CNS-28
// tidal-expect: accept
// tidal-note: The clauses of g are both true, and they are different expressions, so g declares two functions.
template <class T> struct Box {
  static constexpr int f() requires (sizeof(T) > 1) { return 1; }
  static constexpr int f() requires (sizeof(T) <= 1) { return 2; }
  static constexpr int g() requires true { return 1; }
  static constexpr int g() requires (1 == 1) { return 2; }
};
static_assert(Box<int>::f() == 1 && Box<char>::f() == 2);
