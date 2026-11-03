// tidal-rules: JMP-2
// tidal-expect: run
char log_[8];
int n = 0;
struct T { char c; ~T() { log_[n++] = c; } };
int main() {
  for (int i = 0; i < 1; log_[n++] = 's', ++i) {
    T body{'b'};
    {
      T inner{'i'};
      continue;              // inner ends, then body, then the step runs
    }
  }
  return log_[0] == 'i' && log_[1] == 'b' && log_[2] == 's' && n == 3 ? 0 : 1;
}
