// tidal-rules: VC-6
// tidal-expect: run
// tidal-note: The comma operator fixes the order of creation, so the order of destruction at the end of the statement is fixed too.
int log_[4]; int n = 0;
struct Note { int v; ~Note() { log_[n++] = v; } };
int main() {
  int seen = (Note{1}, Note{2}, n);
  log_[n++] = 9;
  return seen == 0 && log_[0] == 2 && log_[1] == 1 && log_[2] == 9 ? 0 : 1;
}
