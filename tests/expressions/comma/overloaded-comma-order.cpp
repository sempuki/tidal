// tidal-rules: COMMA-7
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
char log_[4]; int n = 0;
struct T {};
T operator,(T, T) { return {}; }
T left() { log_[n++] = 'l'; return {}; }
T right() { log_[n++] = 'r'; return {}; }
int main() {
  (left(), right());
  return log_[0] == 'l' && log_[1] == 'r' ? 0 : 1;
}
