template <class F> struct Adaptor { F make; };
int main() {
  auto f = [](int r) { return r * 2; };
  auto a = Adaptor{f};
  return a.make(3) == 6 ? 0 : 1;
}
