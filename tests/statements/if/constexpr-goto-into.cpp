// tidal-rules: IF-17
// tidal-expect: reject
// tidal-error-clang: cannot jump from this goto statement to its label
// tidal-error-gcc: jump to label
// tidal-error-edg: transfer of control into a constexpr if block
void f() {
  goto inside;
  if constexpr (true) {
  inside:;
  }
}
