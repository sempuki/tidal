// tidal-rules: VC-6
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
int log_[4]; int n = 0;
struct Note { int v; ~Note() { log_[n++] = v; } };
int use(const Note&, const Note&) { return n; }
int main() {
  int seen = use(Note{1}, Note{2});
  log_[n++] = 9;
  bool reversed = (log_[0] == 2 && log_[1] == 1) || (log_[0] == 1 && log_[1] == 2);
  return seen == 0 && reversed && log_[2] == 9 ? 0 : 1;
}
