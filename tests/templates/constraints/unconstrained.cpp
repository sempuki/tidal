// tidal-rules: CNS-23
// tidal-expect: accept
template <class T> struct Box {
  static constexpr int get() { return 1; }
  static constexpr int get() requires (sizeof(T) > 1) { return 2; }
};
static_assert(Box<int>::get() == 2 && Box<char>::get() == 1);
