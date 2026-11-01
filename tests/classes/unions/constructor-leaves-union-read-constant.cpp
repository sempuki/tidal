// tidal-rules: UNI-6
// tidal-expect: reject
// tidal-error-clang: union with no active member
// tidal-error-gcc: accessing uninitialized member
// tidal-error-edg: access to uninitialized object
union Slot { int count; float weight; };
struct Holder {
  Slot slot;
  int size;
  constexpr Holder() : size(1) {}
};
constexpr int read_unnamed() { Holder h; return h.slot.count; }
static_assert(read_unnamed() == 0);
