// tidal-rules: PRIM-32 PRIM-31
// tidal-expect: accept
// tidal-note: A hexadecimal parser. The digits are consecutive in every encoding (PRIM-31). The letters a to f are consecutive on the matrix targets (PRIM-32).
constexpr int hex_value(char c) {
  if (c >= '0' && c <= '9') return c - '0';
  if (c >= 'a' && c <= 'f') return c - 'a' + 10;
  if (c >= 'A' && c <= 'F') return c - 'A' + 10;
  return -1;
}
template <int N> constexpr long parse_hex(const char (&s)[N]) {
  long value = 0;
  for (int i = 0; i + 1 < N; ++i) value = value * 16 + hex_value(s[i]);
  return value;
}
static_assert(parse_hex("ff") == 255 && parse_hex("7FfF") == 32767);
static_assert(parse_hex("1a2B3c") == 0x1A2B3C);
static_assert(hex_value('g') == -1 && hex_value('G') == -1);
