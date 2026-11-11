template <class F> struct Adaptor { F make; };
int main() {
  auto a = Adaptor{[](int r) { return r * 2; }};
  return a.make(3) == 6 ? 0 : 1;
}
