// tidal-rules: PRIM-32
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: cannot be represented in the specified character type
// tidal-note: EDG, as configured in the matrix, gives a narrow literal a code unit only for a character that one char holds, so "é" is rejected (PRIM-32).
// tidal-flags: -m32
constexpr char printable[] =
    " !\"#$%&'()*+,-./0123456789:;<=>?"
    "@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_"
    "`abcdefghijklmnopqrstuvwxyz{|}~";
constexpr wchar_t wide[] =
    L" !\"#$%&'()*+,-./0123456789:;<=>?"
    L"@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_"
    L"`abcdefghijklmnopqrstuvwxyz{|}~";
constexpr bool ascii_values() {
  for (int i = 0; i < 95; ++i) {
    if (printable[i] != 32 + i || wide[i] != 32 + i) return false;
  }
  return sizeof(printable) == 96 && sizeof(wide) / sizeof(wchar_t) == 96;
}
static_assert(ascii_values());
static_assert('\0' == 0 && '\a' == 7 && '\b' == 8 && '\t' == 9);
static_assert('\n' == 10 && '\v' == 11 && '\f' == 12 && '\r' == 13);
static_assert(L'\n' == 10 && L'\a' == 7);
// UTF-8: two code units for U+00E9, four for U+1F600.
static_assert(sizeof("é") == 3);
static_assert("é"[0] == static_cast<char>(0xC3));
static_assert("é"[1] == static_cast<char>(0xA9));
static_assert(sizeof("\U0001F600") == 5);
// UTF-32: one code unit for each character.
static_assert(sizeof(L"\U0001F600") == 2 * sizeof(wchar_t));
static_assert(L'é' == 0xE9 && L'\U0001F600' == 0x1F600);
