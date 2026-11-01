// tidal-rules: SEQ-6
// tidal-expect: run
char log_[4]; int n = 0;
struct Total {
  int v = 0;
  Total& operator+=(int k) { v += k; return *this; }
};
Total total;
Total& left() { log_[n++] = 'l'; return total; }
int right() { log_[n++] = 'r'; return 2; }
int main() {
  left() += right();
  return total.v == 2 && log_[0] == 'r' && log_[1] == 'l' ? 0 : 1;
}
