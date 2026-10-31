// tidal-rules: CV-5
// tidal-expect: reject
// tidal-error-gcc: modifying a const object
// tidal-error-clang: modification of object of const-qualified type
// tidal-error-edg: attempt to modify const storage
constexpr int f() {
  const int x = 1;
  const_cast<int&>(x) = 2;
  return x;
}
static_assert(f() == 2);
