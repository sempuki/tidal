// tidal-rules: SEQ-1
// tidal-expect: run
int order[4];
int n = 0;
int mark(int v) { order[n++] = v; return v; }
int second() { return mark(2); }
int take(int a, int b = second()) { return a + b; }
int main() {
  int r = take(mark(1));
  bool either = (order[0] == 1 && order[1] == 2) || (order[0] == 2 && order[1] == 1);
  return r == 3 && either ? 0 : 1;
}
