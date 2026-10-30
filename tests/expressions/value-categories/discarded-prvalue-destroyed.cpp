// tidal-rules: VC-5
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
int log_[4]; int n = 0;
struct Note { int v; ~Note() { log_[n++] = v; } };
Note make(int v) { return Note{v}; }
int main() {
  make(1);
  log_[n++] = 9;
  return log_[0] == 1 && log_[1] == 9 ? 0 : 1;
}
