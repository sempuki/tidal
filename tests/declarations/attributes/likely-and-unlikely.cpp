// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: unlikely. and .likely. attributes are not compatible
// tidal-error-edg: conflicts with previous likely/unlikely attribute
// tidal-note: Gap review 2026-10-30. Q-0046. Clang and EDG reject likely and unlikely on one statement; GCC ignores the second with a warning.
int f(int x) {
  if (x) [[likely]] [[unlikely]] return 1;
  return 0;
}
