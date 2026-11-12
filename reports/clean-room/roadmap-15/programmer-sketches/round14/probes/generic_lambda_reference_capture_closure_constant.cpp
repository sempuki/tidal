template <class F> constexpr int inv(F f) {
  return [&](auto x) { return f(x); }(3);
}
static_assert(inv([](int x) { return x * 2; }) == 6);
int main() {}
