// tidal-rules: LOOP-1
// tidal-expect: run
char log_[8];
int n = 0;
int passes = 0;
struct Cond {
  Cond() {}
  ~Cond() { log_[n++] = 'c'; }
  explicit operator bool() const { return passes++ < 1; }
};
struct Body { ~Body() { log_[n++] = 'b'; } };
int main() {
  while (Cond c = Cond()) { Body b; }
  // pass 1: b ends, then c ends; pass 2: the condition is false, and c ends
  return log_[0] == 'b' && log_[1] == 'c' && log_[2] == 'c' && n == 3 ? 0 : 1;
}
