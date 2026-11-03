// tidal-rules: REF-5
// tidal-expect: accept
// tidal-note: The cast to void names r and does not read the int that r refers to.
struct Cell {
  int v;
  constexpr operator const int&() const { return v; }
};
constexpr int name_only() {
  const int& r = Cell{5};
  (void)r;
  return 1;
}
static_assert(name_only() == 1);
