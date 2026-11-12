struct Buf { int n = 0; };
void put(int v, Buf& b) { b.n += v; }
int main() {
  Buf b;
  auto g = [&](auto v) { put(v, b); };
  g(3);
  return b.n - 3;
}
