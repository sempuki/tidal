// tidal-rules: SEQ-2
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
char log_[4]; int n = 0;
int value = 1;
int& left() { log_[n++] = 'l'; return value; }
int right() { log_[n++] = 'r'; return 2; }
int main() {
  left() += right();
  return value == 3 && log_[0] == 'r' && log_[1] == 'l' ? 0 : 1;
}
