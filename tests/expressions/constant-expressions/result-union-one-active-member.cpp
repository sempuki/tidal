// tidal-rules: CE-21
// tidal-expect: accept
union Slot {
  int number;
  float real;
};
constexpr Slot slot{1};
static_assert(slot.number == 1);
