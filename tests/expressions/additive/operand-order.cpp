// tidal-rules: ADD-13
// tidal-expect: run
int order[2];
int n = 0;
int left() { order[n++] = 1; return 6; }
int right() { order[n++] = 2; return 3; }
int main() {
  int s = left() + right();
  bool either = (order[0] == 1 && order[1] == 2) || (order[0] == 2 && order[1] == 1);
  return s == 9 && either ? 0 : 1;
}
