// tidal-rules: UNI-6
// tidal-expect: reject
// tidal-error-clang: union with no active member
// tidal-error-gcc: accessing uninitialized member
// tidal-error-edg: access to uninitialized object
union Slot { int count; float weight; };
constexpr int read_default() { Slot s; return s.count; }
static_assert(read_default() == 0);
