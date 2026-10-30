// tidal-rules: ADD-14
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-note: Gap review 2026-10-30. Q-0049. GCC accepts arithmetic on a base pointer into an array of derived objects during constant evaluation; Clang and EDG reject it.
struct B { int b; };
struct D : B { int d; };
constexpr int f() {
  D arr[2]{};
  B* p = arr;
  B* q = p + 1;
  return q->b;
}
static_assert(f() == 0);
