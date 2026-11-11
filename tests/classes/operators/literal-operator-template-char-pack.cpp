// tidal-rules: OPR-8 OPR-10
// tidal-expect: run
// tidal-note: Roadmap round 12, programmer reduction literal_operator_template.cpp (from bigint_first_try.cpp). Q-0435. A literal operator template whose template head is one pack of char and whose parameter list is empty takes the characters of an integer or floating literal as template arguments, prefix and point included. Every compiler accepts the declaration and the calls.
template <char... Cs> constexpr int operator""_width() { return sizeof...(Cs); }
template <char... Cs> constexpr char operator""_lead() {
  constexpr char text[] = {Cs...};
  return text[0];
}
static_assert(4096_width == 4);
static_assert(2.5_width == 3);
static_assert(0x1F_width == 4);
int main() { return 731_lead == '7' && 12_width == 2 ? 0 : 1; }
