template <class F, class G>
constexpr auto compose(F f, G g) {
  return [f, g](auto&& x) -> decltype(auto) {
    return f(g(static_cast<decltype(x)&&>(x)));
  };
}
constexpr auto inc = [](int x) { return x + 1; };
constexpr auto id = [](auto&& x) -> decltype(auto) {
  return static_cast<decltype(x)&&>(x);
};
int main() {
  auto h = compose(id, inc);
  int r = h(4);          // the result is a reference to a dead temporary
  return r == 5 ? 0 : 1;
}
