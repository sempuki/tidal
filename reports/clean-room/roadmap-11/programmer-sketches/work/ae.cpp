template <class F> struct Adaptor { F make; };
constexpr auto times(auto k) {
  return Adaptor{[k](int r) { return r * k; }};
}
int main() { return times(2).make(3) == 6 ? 0 : 1; }
