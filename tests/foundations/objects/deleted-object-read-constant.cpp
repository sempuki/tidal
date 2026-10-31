// tidal-rules: OBJ-2
// tidal-expect: reject
// tidal-error-gcc: use of allocated storage after deallocation
// tidal-error-clang: read of heap allocated object that has been deleted
// tidal-error-edg: attempt to access expired storage
constexpr int f() { int* p = new int(3); delete p; return *p; }
static_assert(f() == 3);
