// tidal-rules: COND-2
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
int main() {
  int i = 0;
  int r = (i++ == 0) ? i : -1;
  return r == 1 ? 0 : 1;
}
