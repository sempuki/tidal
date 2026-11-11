int apply(auto k) {
  auto f = [k](auto r) { return r * k; };
  return f(3);
}
int main() { return apply(2) == 6 ? 0 : 1; }
