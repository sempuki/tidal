// tidal-rules: UNI-4 UNI-5
// tidal-expect: accept
union Slot { int count; float weight; };
constexpr int assign_after_default() {
  Slot s;
  s.weight = 2.0f;
  s.count = 7;
  return s.count;
}
static_assert(assign_after_default() == 7);
union Tagged { int count; float weight = 1.5f; };
struct Holder {
  Tagged tag;
  int size;
  constexpr Holder() : size(1) {}
};
constexpr float unnamed_member() { Holder h; return h.tag.weight; }
static_assert(unnamed_member() == 1.5f);
