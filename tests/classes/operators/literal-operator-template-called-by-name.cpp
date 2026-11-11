// tidal-rules: OPR-8 OPR-1
// tidal-expect: run
// tidal-note: Roadmap round 13, programmer report item 1, from front-end report 26 section 5. A program can call a literal operator template by its name with explicit template arguments (OPR-1). The arguments are the characters, as a literal with the suffix gives them (OPR-8).
template <char... Cs> constexpr int operator""_count() { return sizeof...(Cs); }
template <char... Cs> constexpr char operator""_first() {
  constexpr char text[] = {Cs...};
  return text[0];
}
static_assert(operator""_count<'4', '0', '9'>() == 3);
static_assert(operator""_first<'x', 'y'>() == 'x');
int main() {
  if (operator""_count<'1', '2'>() != 12_count) return 1;
  return operator""_first<'7'>() == 70_first ? 0 : 2;
}
