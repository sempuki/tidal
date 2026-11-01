// tidal-rules: ADD-14 ADD-10
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: cannot refer to element 2 of non-array object
// tidal-error-edg: invalid arithmetic on non-array pointer
// tidal-note: Q-0049. GCC accepts moving a base pointer two positions during constant evaluation. Clang and EDG reject it.
struct B { int b; };
struct D : B { int d; };
constexpr bool f() {
  D arr[3]{};
  B* p = arr;
  B* q = p + 2;
  return q != nullptr;
}
static_assert(f());
