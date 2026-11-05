// tidal-rules: LAM-28
// tidal-expect: run
// tidal-note: Init captures only. The order is unspecified (LAM-28). Every compiler of the matrix runs each initializer in the order of the capture list, and the test records that order.
int order[3];
int n = 0;
int mark(int v) { order[n++] = v; return v; }
int main() {
  auto g = [z = mark(3), x = mark(1), y = mark(2)] { return x + y + z; };
  if (n != 3 || order[0] != 3 || order[1] != 1 || order[2] != 2) return 1;
  return g() == 6 ? 0 : 2;
}
