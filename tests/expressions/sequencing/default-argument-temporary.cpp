// tidal-rules: SEQ-5
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
int log_[4]; int n = 0;
struct Note { int v; ~Note() { log_[n++] = v; } };
int f(const Note& note = Note{7}) { return note.v; }
int main() {
  int r = f();
  log_[n++] = 9;
  return r == 7 && log_[0] == 7 && log_[1] == 9 ? 0 : 1;
}
