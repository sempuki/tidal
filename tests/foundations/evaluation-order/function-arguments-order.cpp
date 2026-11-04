// tidal-rules: SEQ-1
// tidal-expect: run
int order[2];
int n = 0;
int a() { order[n++] = 1; return 1; }
int b() { order[n++] = 2; return 2; }
int sum(int x, int y) { return x + y; }
int main() {
  int r = sum(a(), b());
  bool either = (order[0] == 1 && order[1] == 2) || (order[0] == 2 && order[1] == 1);
  return r == 3 && either ? 0 : 1;
}
