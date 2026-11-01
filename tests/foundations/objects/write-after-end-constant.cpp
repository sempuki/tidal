// tidal-rules: OBJ-2
// tidal-expect: reject
// tidal-error-clang: assignment to object outside its lifetime
// tidal-error-gcc: modification of .x. outside its lifetime
// tidal-error-edg: attempt to access expired storage
constexpr int f() {
  int* p = nullptr;
  {
    int x = 1;
    p = &x;
  }
  *p = 2;
  return 0;
}
static_assert(f() == 0);
