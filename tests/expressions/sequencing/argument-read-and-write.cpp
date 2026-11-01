// tidal-rules: SEQ-1
// tidal-expect: run
int pair(int x, int y) { return x * 10 + y; }
int main() {
  int i = 0;
  int r = pair(i++, i);
  return (r == 1 || r == 0) && i == 1 ? 0 : 1;
}
