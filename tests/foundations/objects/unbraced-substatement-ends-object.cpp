// tidal-rules: OBJ-3
// tidal-expect: run
char log_[16];
int n = 0;
struct T {
  char c;
  T(char x) : c(x) { log_[n++] = x; }
  ~T() { log_[n++] = c - 32; }   // the upper-case letter marks the end
};
int main() {
  bool yes = true;
  if (yes) T a('a');             // a branch without braces is a scope
  log_[n++] = '|';
  for (int i = 0; i < 2; ++i) T w('w');   // so is a loop body
  return log_[0] == 'a' && log_[1] == 'A' && log_[2] == '|' &&
         log_[3] == 'w' && log_[4] == 'W' && log_[5] == 'w' && log_[6] == 'W' && n == 7 ? 0 : 1;
}
