// tidal-rules: CMP-20
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
int a[1], b[1];
int main() {
  bool r = a + 1 == b;
  return (r || !r) ? 0 : 1;
}
