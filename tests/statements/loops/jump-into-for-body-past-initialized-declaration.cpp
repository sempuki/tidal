// tidal-rules: JMP-6 LOOP-9
// tidal-expect: reject
// tidal-error-clang: jump bypasses variable initialization|crosses initialization|protected scope
// tidal-error-gcc: crosses initialization
// tidal-error-edg: bypasses initialization
// tidal-note: G161. The jump to case 1 enters the for body and skips the initializer int i = 0, which declares a variable that is in scope at the label.
int run(int k) {
  int sum = 0;
  switch (k) {
    case 0:
      for (int i = 0; i < 3; ++i) {
        case 1:
        sum += i;
      }
  }
  return sum;
}
