// tidal-rules: COMMA-1
// tidal-expect: run
int order[2];
int n = 0;
int a() { order[n++] = 1; return 10; }
int b() { order[n++] = 2; return 20; }
int main() {
  int r = (a(), b());
  return r == 20 && order[0] == 1 && order[1] == 2 ? 0 : 1;
}
