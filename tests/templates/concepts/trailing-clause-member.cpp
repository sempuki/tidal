// tidal-rules: CPT-11
// tidal-expect: accept
template <class T> struct Box {
  constexpr int get() requires (sizeof(T) > 1) { return 1; }
};
template <class T> struct Outer {
  struct Inner { constexpr int get() requires (sizeof(T) > 1) { return 2; } };
};
static_assert(Box<int>{}.get() == 1);
static_assert(Outer<int>::Inner{}.get() == 2);
