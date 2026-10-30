// tidal-rules: LIST-6
// tidal-expect: accept
constexpr char a[] = "ab";
constexpr char b[4] = "ab";
constexpr char c[] = {"ab"};
static_assert(sizeof(a) == 3 && a[2] == 0);
static_assert(b[3] == 0);
static_assert(sizeof(c) == 3);
constexpr wchar_t w[] = L"x";
constexpr char16_t u[] = u"x";
static_assert(sizeof(u) == 2 * sizeof(char16_t));
