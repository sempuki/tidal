template <class F> struct Adaptor { F make; };
constexpr auto twice(int k) {
  return Adaptor{[k](int r) { return r * k; }};
}
int main() { return twice(2).make(3) == 6 ? 0 : 1; }
