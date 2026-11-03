// tidal-rules: JMP-6
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-expect-edg: reject
// tidal-error-clang: cannot jump from this goto statement to its label
// tidal-error-edg: transfer of control bypasses initialization
// tidal-note: Gap review 2026-10-30. Q-0041. The class has a trivial default constructor, so default initialization does nothing and the jump skips nothing (JMP-6). The destructor plays no part. GCC accepts the jump. Clang and EDG reject it.
struct Tidy { int v; ~Tidy() {} };
int f() {
  goto skip;
  Tidy t;
skip:
  return 0;
}
