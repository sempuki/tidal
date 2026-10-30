// tidal-rules: SEQ-6
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
char log_[4]; int n = 0;
struct V { V& operator=(int) { return *this; } };
V v;
V& left() { log_[n++] = 'l'; return v; }
int right() { log_[n++] = 'r'; return 1; }
int main() {
  left() = right();
  return log_[0] == 'r' && log_[1] == 'l' ? 0 : 1;
}
