// tidal-rules: PRIM-33
// tidal-expect: accept
// tidal-note: The simple escapes in string literals. Each escape is one element of the array.
constexpr char controls[] = "\a\b\t\n\v\f\r";
static_assert(sizeof controls == 8);
static_assert(controls[0] == 7 && controls[1] == 8 && controls[2] == 9 && controls[3] == 10);
static_assert(controls[4] == 11 && controls[5] == 12 && controls[6] == 13);
constexpr wchar_t marks[] = L"\'\"\?\\";
static_assert(sizeof marks / sizeof marks[0] == 5);
static_assert(marks[0] == 39 && marks[1] == 34 && marks[2] == 63 && marks[3] == 92);
static_assert(u"\n"[0] == 10 && u8"\\"[0] == 92);
