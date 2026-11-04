// tidal-rules: CE-2
// tidal-expect: accept
// tidal-note: The implicit constructor, copy constructor, and destructor of Holder call only constexpr functions, so constant evaluation can run them.
struct Part {
  int v;
  constexpr Part() : v(1) {}
};
struct Holder {
  Part part;
  int extra = 2;
};
constexpr int total() {
  Holder h;
  Holder copy = h;
  return copy.part.v + copy.extra;
}
static_assert(total() == 3);
