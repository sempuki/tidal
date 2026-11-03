// tidal-rules: SEQ-2
// tidal-expect: run
char log_[4]; int n = 0;
int a, b;
int& x() { log_[n++] = 'x'; return a; }
int& y() { log_[n++] = 'y'; return b; }
int z() { log_[n++] = 'z'; return 5; }
int main() {
  x() = y() = z();
  return a == 5 && b == 5 && log_[0] == 'z' && log_[1] == 'y' && log_[2] == 'x' ? 0 : 1;
}
