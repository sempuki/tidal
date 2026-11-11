// tidal-rules: OPR-8 PRIM-2
// tidal-expect: run
// tidal-note: Roadmap round 12, programmer reduction raw_literal_operator_long_literal.cpp (from bigint_first_try.cpp). Q-0436. The literal has the suffix of a raw literal operator, and no integer type holds its 25 digits. The literal operator gets the digits as a string (OPR-8), so PRIM-2 does not apply. Every compiler accepts.
constexpr int count_digits(const char* text) {
  int n = 0;
  while (text[n] != '\0') ++n;
  return n;
}
constexpr int operator""_digits(const char* text) { return count_digits(text); }
static_assert(9999999999999999999999999_digits == 25);
int main() { return 1234567890123456789012345678_digits == 28 ? 0 : 1; }
