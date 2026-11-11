// A generic lambda that captures a pack by copy and expands it in a call.
template <class F, class... X> auto bind_front(F f, X... xs) {
  return [f, xs...](auto... rest) { return f(xs..., rest...); };
}
int main() {
  auto p = bind_front([](int a, int b, int c) { return a * 100 + b * 10 + c; }, 1, 2);
  return p(3) == 123 ? 0 : 1;
}
