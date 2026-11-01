// tidal-rules: ADD-14 ADD-6
// tidal-expect: accept
struct B { int b; };
struct D : B { int d; };
constexpr bool f() {
  D arr[2]{};
  B* p = arr;
  B* q = p + 1;
  return q != nullptr && q - p == 1;
}
static_assert(f());
