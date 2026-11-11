// tidal-rules: OPR-1 OPR-8
// tidal-expect: run
// tidal-note: Ledger row F6-2, from front-end report 26, section 5. A call of a literal operator template by its name gives the characters as explicit template arguments, and gives the value of the literal with the same characters. An empty list, or none, gives an empty pack. Every compiler accepts the program.
template <char... Cs> constexpr int operator""_weigh() {
  return static_cast<int>(sizeof...(Cs)) * 10 + ((Cs - '0') + ... + 0);
}
static_assert(12_weigh == 23);
static_assert(operator""_weigh<'1', '2'>() == 23);
static_assert(operator""_weigh<'1', '2'>() == 12_weigh);
static_assert(operator""_weigh<>() == 0);
static_assert(operator""_weigh() == 0);
int main() {
  int by_name = operator""_weigh<'3', '4', '5'>();
  return by_name == 345_weigh && by_name == 42 ? 0 : 1;
}
