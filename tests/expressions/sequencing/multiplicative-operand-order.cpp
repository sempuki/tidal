// tidal-rules: SEQ-8
// tidal-expect: run
int order[2];
int n = 0;
int a() { order[n++] = 1; return 1; }
int b() { order[n++] = 2; return 2; }
int main() {
  auto product = a() * b();
  (void)product;
  bool either = (order[0] == 1 && order[1] == 2) || (order[0] == 2 && order[1] == 1);
  return n == 2 && either ? 0 : 1;
}
