// tidal-rules: IF-17
// tidal-expect: reject
// tidal-error-clang: cannot jump from this goto statement to its label
// tidal-error-gcc: jump to label
// tidal-error-edg: transfer of control into an .if consteval
void f() {
  goto inside;
  if consteval {
  inside:;
  }
}
