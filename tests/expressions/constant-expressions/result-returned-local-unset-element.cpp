// tidal-rules: CE-21 CE-22
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: subobject .v. is not initialized
// tidal-error-edg: uninitialized
// tidal-note: Q-0115. Write report W11. The builder sets three of four elements and returns its local. Clang and EDG reject the constant. GCC accepts it.
struct Table { int v[4]; };
constexpr Table make() {
  Table t;
  for (int i = 0; i < 3; ++i) t.v[i] = i + 1;
  return t;
}
constexpr Table table = make();
