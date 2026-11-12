// tidal-rules: UNEV-3 LOOP-3
// tidal-expect: reject
// tidal-error-clang: expression result unused
// tidal-error-gcc: for increment expression has no effect
// tidal-flags: -Werror -Wunused-value
// tidal-note: X26-2. The iteration expression is a name, a discarded-value expression with no effect. Clang and GCC warn as for the same expression statement.
int count_up(int limit) {
  int k = 0;
  for (; k < limit; limit) ++k;
  return k;
}
