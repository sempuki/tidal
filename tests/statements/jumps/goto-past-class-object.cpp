// tidal-rules: JMP-6
// tidal-expect: reject
struct S { S() {} };
void f() {
  goto skip;
  S s;
skip:;
}
