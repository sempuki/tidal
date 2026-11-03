// tidal-rules: COND-7
// tidal-expect: accept
// tidal-note: The operands that are not selected dereference a null pointer and read past the end of an array. Neither is evaluated.
constexpr int read(const int* p) { return p ? *p : 0; }
static_assert(read(nullptr) == 0);
constexpr int values[2] = {5, 6};
constexpr int at(int i) { return i < 2 ? values[i] : values[i]; }
static_assert(at(1) == 6);
constexpr int last(int i) { return i >= 2 ? -1 : values[i]; }
static_assert(last(2) == -1);
