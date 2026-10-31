// tidal-rules: CE-21
// tidal-expect: accept
struct Table {
  int slots[2];
  constexpr Table() { slots[0] = 1; slots[1] = 2; }
};
constexpr Table table;
static_assert(table.slots[1] == 2);
