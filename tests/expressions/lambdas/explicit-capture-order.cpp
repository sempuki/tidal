// tidal-rules: LAM-28 LAM-7
// tidal-expect: run
// tidal-note: The order is unspecified (LAM-28). Every compiler of the matrix initializes the explicit captures in the order of the capture list, and the test records that order.
int order[4];
int n = 0;
struct Tag {
  int v;
  Tag(int x) : v(x) {}
  Tag(const Tag& from) : v(from.v) { order[n++] = v; }
};
int mark(int v) { order[n++] = v; return v; }
int main() {
  Tag a{1}, b{2}, c{3};
  n = 0;
  auto copies = [c, a, b] { return a.v + b.v + c.v; };
  if (n != 3 || order[0] != 3 || order[1] != 1 || order[2] != 2) return 1;
  n = 0;
  auto mixed = [b, x = mark(7), a] { return a.v + b.v + x; };
  if (n != 3 || order[0] != 2 || order[1] != 7 || order[2] != 1) return 2;
  return copies() == 6 && mixed() == 10 ? 0 : 3;
}
