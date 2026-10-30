// tidal-rules: INIT-1
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
int main() {
  unsigned char a;
  unsigned char b = a;
  (void)b;
  return 0;
}
