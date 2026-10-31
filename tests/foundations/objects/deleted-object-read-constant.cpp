// tidal-rules: OBJ-2
// tidal-expect: reject
// tidal-error-gcc: use of allocated storage after deallocation
constexpr int f() { int* p = new int(3); delete p; return *p; }
static_assert(f() == 3);
