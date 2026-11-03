// tidal-rules: CLS-15
// tidal-expect: accept
// tidal-note: A static operator[] with no parameters, and a static operator() named through the class.
struct Table {
  static constexpr int operator[]() { return 7; }
  static constexpr int operator()(int x) { return x + 1; }
};
constexpr Table t{};
static_assert(t[] == 7);
static_assert(Table::operator()(4) == 5);
static_assert(Table{}(1) == 2);
