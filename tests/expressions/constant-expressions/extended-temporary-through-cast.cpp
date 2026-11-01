// tidal-rules: CE-14
// tidal-expect: accept
constexpr const int& cast = static_cast<const int&>(4);
constexpr const int& chosen = true ? 5 : 6;
constexpr const int& last = (0, 7);
constexpr const int* where = &cast;
static_assert(where == &cast);
