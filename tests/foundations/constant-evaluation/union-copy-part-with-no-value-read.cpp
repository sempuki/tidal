// tidal-rules: CE-22
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: read of uninitialized object|not an integral constant expression
// tidal-error-gcc: non-constant condition|uninitialized
// tidal-note: Ledger row X25-5. The trivial copy of the union reads no part, so the element items[1], which has no value in the source, has no value in the copy, and the read of it is rejected. Clang and GCC reject the read. EDG accepts it (Q-0448).
union Cells {
  int items[2];
  long other;
  constexpr Cells() {}
};
constexpr int read_copy() {
  Cells source;
  source.items[0] = 1;
  Cells copied = source;
  return copied.items[1];
}
static_assert(read_copy() == 0);
