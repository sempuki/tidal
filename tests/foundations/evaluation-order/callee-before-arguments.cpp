// tidal-rules: SEQ-2
// tidal-expect: run
char log[4];
int pos = 0;
int target(int v) { return v; }
int (*pick())(int) { log[pos++] = 'f'; return target; }
int arg() { log[pos++] = 'a'; return 3; }
int main() {
  int r = pick()(arg());
  return r == 3 && log[0] == 'f' && log[1] == 'a' ? 0 : 1;
}
