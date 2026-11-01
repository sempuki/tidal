// tidal-rules: LAM-20
// tidal-expect: run
int order[4];
int n = 0;
struct Tag {
  int v;
  Tag(int x) : v(x) {}
  Tag(const Tag& from) : v(from.v) { order[n++] = v; }
};
int mark(int v) { order[n++] = v; return v; }
bool seen(int v) {
  int count = 0;
  for (int i = 0; i < n; ++i) count += order[i] == v;
  return count == 1;
}
int main() {
  Tag a{1}, b{2};
  auto f = [=, x = mark(7)] { return b.v + x + a.v; };
  // Each capture is initialized once, in an unspecified order.
  if (n != 3 || !seen(1) || !seen(2) || !seen(7)) return 1;
  return f() == 10 ? 0 : 2;
}
