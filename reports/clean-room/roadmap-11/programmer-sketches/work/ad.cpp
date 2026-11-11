template <class F> struct Adaptor { F make; };
template <class F> constexpr auto operator|(int r, const Adaptor<F>& a) { return a.make(r); }
constexpr auto times(int k) {
  return Adaptor{[k](auto r) { return r * k; }};
}
int main() { return (3 | times(2)) == 6 ? 0 : 1; }
