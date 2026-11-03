// tidal-rules: SB-17
// tidal-expect: accept
// tidal-note: Write report W6. An lvalue result of get gives a hidden lvalue reference, so the name refers to the member. A prvalue result gives a hidden rvalue reference, also under auto&.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
namespace std {
template <class T> struct tuple_size;
template <decltype(sizeof 0) I, class T> struct tuple_element;
}
struct Cell {
  int value;
  template <decltype(sizeof 0) I> constexpr int& get() { return value; }
};
template <> struct std::tuple_size<Cell> { static constexpr decltype(sizeof 0) value = 1; };
template <> struct std::tuple_element<0, Cell> { using type = int; };
constexpr int write_through() {
  Cell c{1};
  auto& [v] = c;
  v = 5;              // the hidden reference is int&, bound to c.value
  return c.value;
}
static_assert(write_through() == 5);

struct Made {
  template <decltype(sizeof 0) I> constexpr int get() const { return int(I) + 10; }
};
template <> struct std::tuple_size<Made> { static constexpr decltype(sizeof 0) value = 2; };
template <decltype(sizeof 0) I> struct std::tuple_element<I, Made> { using type = int; };
constexpr int from_prvalue() {
  Made m;
  auto& [a, b] = m;   // each get result is a prvalue; the hidden reference is int&&
  static_assert(same<decltype(a), int>);
  return a + b;
}
static_assert(from_prvalue() == 21);
