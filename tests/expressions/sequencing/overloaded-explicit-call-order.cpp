// tidal-rules: SEQ-6 SEQ-1
// tidal-expect: run
// tidal-note: The operator notation evaluates the left operand first. An explicit call of the same operator function is a function call, and its arguments can come in either order.
char log_[4]; int n = 0;
struct Out {};
Out out;
Out& operator<<(Out& o, int) { return o; }
Out& left() { log_[n++] = 'L'; return out; }
int right() { log_[n++] = 'R'; return 1; }
int main() {
  left() << right();
  bool notation = log_[0] == 'L' && log_[1] == 'R';
  operator<<(left(), right());
  bool call = (log_[2] == 'L' && log_[3] == 'R') || (log_[2] == 'R' && log_[3] == 'L');
  return notation && call ? 0 : 1;
}
