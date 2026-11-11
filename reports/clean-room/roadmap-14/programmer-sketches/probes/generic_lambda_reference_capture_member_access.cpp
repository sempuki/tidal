struct Buf { int n = 0; };
int main() {
  Buf b;
  auto g = [&](auto v) { b.n += v; };
  g(3);
  return b.n - 3;
}
