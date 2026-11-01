// tidal-rules: CTAD-1
// tidal-expect: accept
// tidal-note: The variable of a range-based for loop and a variable declared in a condition have initializers, so each can have a deduced class template type.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class A, class B> struct Pair { A a; B b; };
template <class T> struct Box {
  T v;
  constexpr Box(T x) : v(x) {}
  constexpr explicit operator bool() const { return v != 0; }
};
constexpr int loop() {
  Pair<int, char> items[2] = {{1, 'a'}, {2, 'b'}};
  int total = 0;
  for (Pair p : items) {
    static_assert(same<decltype(p), Pair<int, char>>);
    total += p.a;
  }
  return total;
}
constexpr int condition() {
  if (Box b = Box(2)) {
    static_assert(same<decltype(b), Box<int>>);
    return b.v;
  }
  return 0;
}
constexpr int braced_condition() {
  if (Box b{3}) return b.v;
  return 0;
}
static_assert(loop() == 3 && condition() == 2 && braced_condition() == 3);
