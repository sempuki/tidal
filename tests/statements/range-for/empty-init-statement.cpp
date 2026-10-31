// tidal-rules: RFOR-3
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: in nested-name-specifier
// tidal-note: Q-0122. Clang and EDG accept an empty init statement in a range-based for; GCC rejects it.
int sum(const int (&v)[3]) {
  int s = 0;
  for (; int x : v) s += x;
  return s;
}
