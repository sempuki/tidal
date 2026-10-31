// tidal-rules: CE-22 OBJ-2
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: subobject .second. is not initialized
// tidal-error-edg: access to uninitialized object
// tidal-note: Q-0115. Clang and EDG reject the copy because it reads the member that has no value; GCC accepts it.
struct Pair {
  int first;
  int second;
  constexpr Pair() : first(1) {}
};
constexpr int copy_first() {
  Pair original;
  Pair copy = original;
  return copy.first;
}
static_assert(copy_first() == 1);
