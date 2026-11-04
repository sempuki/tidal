// tidal-rules: UNI-7
// tidal-expect: accept
// tidal-note: In the braced list for Reading, the anonymous union is one element, and its braces can be left out. The 2 initializes the first member of the union.
struct Reading {
  int id;
  union { int count; float level; };
  int unit;
};
constexpr Reading r = {1, 2, 3};
static_assert(r.id == 1 && r.count == 2 && r.unit == 3);
