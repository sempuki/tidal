// tidal-rules: VC-5
// tidal-expect: run
// tidal-note: The expression statement is a braced class prvalue, not a call.
int log_[4]; int n = 0;
struct Note { int v; ~Note() { log_[n++] = v; } };
int main() {
  Note{2};
  log_[n++] = 9;
  return n == 2 && log_[0] == 2 && log_[1] == 9 ? 0 : 1;
}
