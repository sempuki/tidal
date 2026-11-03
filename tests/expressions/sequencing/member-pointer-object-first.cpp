// tidal-rules: SEQ-2
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
char log_[4]; int n = 0;
struct S { int v = 4; };
S s;
S& obj() { log_[n++] = 'o'; return s; }
int S::* member() { log_[n++] = 'm'; return &S::v; }
int main() {
  int r = obj().*member();
  return r == 4 && log_[0] == 'o' && log_[1] == 'm' ? 0 : 1;
}
