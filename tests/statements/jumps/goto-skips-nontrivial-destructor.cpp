// tidal-rules: JMP-6
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-expect-edg: reject
// tidal-error-clang: cannot jump from this goto statement to its label
// tidal-error-edg: transfer of control bypasses initialization
// tidal-note: Q-0041. The class has a trivial default constructor, so default initialization does nothing and the jump skips nothing (JMP-6). The destructor plays no part. GCC accepts the jump. Clang and EDG reject it.
// tidal-note: Gap review 2026-10-31 (third).
struct D { int a; ~D() {} };
int f() { goto skip; D d; skip: return 0; }
