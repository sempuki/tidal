// tidal-rules: IF-17
// tidal-expect: reject
// tidal-error-clang: cannot jump from switch statement to this case label
// tidal-error-gcc: jump to case label
// tidal-error-edg: transfer of control into a constexpr if block
void f(int k) {
  switch (k) {
    case 0:
      if constexpr (true) {
    case 1:;
      }
  }
}
