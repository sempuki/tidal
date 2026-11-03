// tidal-rules: CE-32 CE-21
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: subobject .v. is not initialized
// tidal-error-edg: uninitialized
// tidal-note: Q-0115. Write report W11. GCC accepts the constant and reads the unset element as 0.
struct Table { int v[4]; };
constexpr Table make() {
  Table t;
  for (int i = 0; i < 3; ++i) t.v[i] = i + 1;
  return t;
}
constexpr Table table = make();
static_assert(table.v[3] == 0);
