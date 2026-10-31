// tidal-rules: CMP-20
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-gcc: != \(\(int\*\)\(& b\)\)\). is not a constant expression
// tidal-note: Gap review 2026-10-30. Q-0051. EDG accepts comparing a one-past-the-end pointer with the next object during constant evaluation; Clang and GCC reject it.
int a[1], b[1];
static_assert(a + 1 != b);
