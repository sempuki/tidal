// tidal-rules: ATTR-8
// tidal-expect: run
// tidal-note: [[unlikely]] on the body of a loop and [[likely]] on a return statement. The program runs as without the attributes.
int main() {
  int n = 0;
  while (n < 3) [[unlikely]] ++n;
  [[likely]] return n == 3 ? 0 : 1;
}
