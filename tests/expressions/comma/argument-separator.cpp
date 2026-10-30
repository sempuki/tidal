// tidal-rules: COMMA-4
// tidal-expect: run
int count(int, int) { return 2; }
int count(int) { return 1; }
int main() {
  return count(1, 2) == 2 && count((1, 2)) == 1 ? 0 : 1;
}
