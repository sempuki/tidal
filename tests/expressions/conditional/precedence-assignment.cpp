// tidal-rules: COND-6
// tidal-expect: run
int main() {
  int x = 0, y = 0;
  true ? x = 1 : y = 2;
  return x == 1 && y == 0 ? 0 : 1;
}
