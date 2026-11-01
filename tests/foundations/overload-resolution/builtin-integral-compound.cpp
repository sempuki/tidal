// tidal-rules: OVL-9
// tidal-expect: accept
// tidal-note: The left operand of %=, &=, and <<= can have any integral type, including bool and the character types. The class operand converts to int.
struct ToInt { constexpr operator int() const { return 3; } };
constexpr bool probe() {
  char c = 10;
  c %= ToInt{};
  wchar_t w = 1;
  w <<= ToInt{};
  bool b = true;
  b &= ToInt{};
  return c == 1 && w == 8 && b;
}
static_assert(probe());
