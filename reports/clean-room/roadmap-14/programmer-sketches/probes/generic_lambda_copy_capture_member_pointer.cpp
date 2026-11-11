struct W { int len; int twice() const { return 2 * len; } };
int main() {
  auto f = &W::twice;
  auto g = [f](auto&& o) { return (o.*f)(); };
  return g(W{3}) - 6;
}
