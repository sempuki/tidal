// tidal-rules: OBJ-2
// tidal-expect: reject
constexpr int f() { int* p = new int(3); delete p; return *p; }
static_assert(f() == 3);
