// tidal-rules: SEQ-6
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
char log_[4]; int n = 0;
struct Out { Out& operator<<(int) { return *this; } };
Out out;
int first() { log_[n++] = '1'; return 1; }
int second() { log_[n++] = '2'; return 2; }
int main() {
  out << first() << second();
  return log_[0] == '1' && log_[1] == '2' ? 0 : 1;
}
