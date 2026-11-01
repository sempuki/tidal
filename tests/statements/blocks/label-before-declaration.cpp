// tidal-rules: STMT-5
// tidal-expect: run
int main() {
  int n = 0;
  goto next;
next:
  int x = 4;
  n += x;
  return n == 4 ? 0 : 1;
}
