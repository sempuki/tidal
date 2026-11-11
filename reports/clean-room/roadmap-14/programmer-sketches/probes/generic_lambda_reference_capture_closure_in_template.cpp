template <class F> constexpr int call3(F f) {
  auto g = [&](auto x) { return f(x); };
  return g(3);
}
int main() { return call3([](int x) { return 2 * x; }) - 6; }
