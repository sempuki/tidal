// tidal-rules: LAM-7
// tidal-expect: run
int order[4];
int n = 0;
int mark(int v) { order[n++] = v; return v; }
int main() {
  auto f = [x = mark(1), y = mark(2)] { return x + y; };
  return f() == 3 && order[0] == 1 && order[1] == 2 ? 0 : 1;
}
