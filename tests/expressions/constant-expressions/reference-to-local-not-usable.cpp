// tidal-rules: CE-29
// tidal-expect: reject
// tidal-error-clang: must be initialized by a constant expression
// tidal-error-gcc: is not a constant expression|not usable in a constant expression
// tidal-error-edg: must have a constant value
// tidal-note: table has automatic storage, so the initializer of view is not a constant expression (CE-14), and view is not usable in constant expressions.
int use() {
  constexpr int table[] = {1, 2};
  auto&& view = table;
  constexpr int second = view[1];
  return second;
}
