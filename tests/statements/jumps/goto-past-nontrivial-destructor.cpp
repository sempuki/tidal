// tidal-rules: JMP-6
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: cannot jump from this goto statement to its label
// tidal-error-edg: transfer of control bypasses initialization
// tidal-note: Gap review 2026-10-30. Q-0041. GCC accepts a jump past a class object with a non-trivial destructor.
struct Tidy { int v; ~Tidy() {} };
int f() {
  goto skip;
  Tidy t;
skip:
  return 0;
}
