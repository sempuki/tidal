// tidal-rules: CE-21
// tidal-expect: reject
// tidal-error-clang: subobject .slots. is not initialized
// tidal-error-gcc: incompletely initialized variable
// tidal-error-edg: access to uninitialized
struct Table {
  int slots[2];
  constexpr Table() { slots[0] = 1; }
};
constexpr Table table;
