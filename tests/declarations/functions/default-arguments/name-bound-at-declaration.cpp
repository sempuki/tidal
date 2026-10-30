// tidal-rules: DEFARG-3
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
int limit = 3;
int f(int v = limit) { return v; }
int main() {
  int limit = 9;
  (void)limit;
  return f() == 3 ? 0 : 1;
}
