// tidal-rules: STMT-6
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
int main() {
  int x = 1;
  {
    int(x);
    x = 5;
  }
  return x == 1 ? 0 : 1;
}
