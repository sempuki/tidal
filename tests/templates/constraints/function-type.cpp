// tidal-rules: CNS-30
// tidal-expect: accept
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> struct Box {
  static int get(int) requires (sizeof(T) > 1) { return 1; }
};
static_assert(same<decltype(&Box<int>::get), int (*)(int)>);
