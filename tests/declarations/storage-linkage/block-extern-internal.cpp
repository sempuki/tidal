// tidal-rules: LNK-9
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
static int hidden = 7;
int read() {
  extern int hidden;
  return hidden;
}
int main() { return read() == 7 ? 0 : 1; }
