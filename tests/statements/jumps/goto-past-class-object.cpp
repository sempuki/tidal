// tidal-rules: JMP-6
// tidal-expect: reject
// tidal-error-clang: cannot jump from this goto statement to its label
// tidal-error-gcc: jump to label
// tidal-error-edg: transfer of control bypasses initialization
struct S { S() {} };
void f() {
  goto skip;
  S s;
skip:;
}
