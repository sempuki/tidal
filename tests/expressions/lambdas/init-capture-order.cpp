// tidal-rules: LAM-28
// tidal-expect: run
// tidal-note: Init captures only. Each initializer runs in the order of the capture list.
int order[3];
int n = 0;
int mark(int v) { order[n++] = v; return v; }
int main() {
  auto g = [z = mark(3), x = mark(1), y = mark(2)] { return x + y + z; };
  if (n != 3 || order[0] != 3 || order[1] != 1 || order[2] != 2) return 1;
  return g() == 6 ? 0 : 2;
}
