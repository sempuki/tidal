using size_t = decltype(sizeof 0);
namespace std { template <class T> struct tuple_size; template <size_t I, class T> struct tuple_element; }
struct R { int a, b; template <size_t I> constexpr int get() const { return I ? b : a; } };
template <> struct std::tuple_size<R> { static constexpr size_t value = 2; };
template <size_t I> struct std::tuple_element<I, R> { using type = int; };
template <> struct std::tuple_size<const R> { static constexpr size_t value = 2; };
template <size_t I> struct std::tuple_element<I, const R> { using type = int; };
template <class A, class B> constexpr bool same_type = false;
template <class A> constexpr bool same_type<A, A> = true;
int main() {
  const auto& [x, y] = R{1, 2};
  static_assert(same_type<decltype(x), int>);
  return x + y - 3;
}
