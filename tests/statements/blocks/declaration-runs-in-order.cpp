// tidal-rules: STMT-3
// tidal-expect: run
int order[4];
int n = 0;
int mark(int v) { order[n++] = v; return v; }
int main() {
  mark(1);
  int a = mark(2);
  { int b = mark(3); (void)b; }
  mark(4);
  return a == 2 && order[0] == 1 && order[1] == 2 && order[2] == 3 && order[3] == 4 ? 0 : 1;
}
