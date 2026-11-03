namespace std {
using size_t = decltype(sizeof 0);
template <class T> struct tuple_size;
template <size_t I, class T> struct tuple_element;
}
template <class A, class B> constexpr bool same_type = false;
template <class A> constexpr bool same_type<A, A> = true;
class Range {
  int lo_, hi_;
public:
  constexpr Range(int lo, int hi) : lo_(lo), hi_(hi) {}
  template <std::size_t I> constexpr int get() const { return I == 0 ? lo_ : hi_; }
};
namespace std {
template <> struct tuple_size<Range> { static constexpr size_t value = 2; };
template <size_t I> struct tuple_element<I, Range> { using type = int; };
#ifdef CONSTSPEC
template <> struct tuple_size<const Range> { static constexpr size_t value = 2; };
template <size_t I> struct tuple_element<I, const Range> { using type = const int; };
#endif
}
constexpr int byvalue() {
  auto [a, b] = Range{1, 2};     // get returns a prvalue int
  static_assert(same_type<decltype(a), int>);
  a = 7;                         // SB-12: an lvalue
  return a + b;
}
static_assert(byvalue() == 9);
#ifdef CONST
constexpr int constbind() {
  const auto [a, b] = Range{1, 2};
  return a + b;
}
static_assert(constbind() == 3);
#endif
#ifdef LREF
constexpr int lref() {
  Range r{1, 2};
  auto& [a, b] = r;   // get returns prvalue; hidden reference?
  return a + b;
}
static_assert(lref() == 3);
#endif
int main() {}
