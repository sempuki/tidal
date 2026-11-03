// tidal-rules: OBJ-10
// tidal-expect: reject
// tidal-error-clang: read of member .* of union with active member
// tidal-error-gcc: accessing .U::a. member instead of (initialized|active) .U::b. member
// tidal-error-edg: invalid access to inactive member
// tidal-note: The old object is a member of a struct in a union. An assignment to the other member reuses the storage, and a pointer to the old member is read.
struct A { int x; };
struct B { float y; };
union U { A a; B b; };
constexpr int f() {
  U u{.a = {1}};
  const int* p = &u.a.x;
  u.b = B{2.0f};
  return *p;
}
static_assert(f() == 1);
