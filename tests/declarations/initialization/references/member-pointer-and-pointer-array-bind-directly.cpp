// tidal-rules: REF-1
// tidal-expect: run
// tidal-note: Decision 0059, item RV5, point RV5-A-5. A pointer to member binds a reference to a pointer to member of a more qualified type directly, and an array of int* binds a reference to an array of const int* const directly. Each reference and its lvalue are one object.
struct Cell { int v; };
int x = 0;
int Cell::* member = &Cell::v;
int* row[2] = {&x, &x};
int main() {
  const int Cell::* const& m = member;
  const int* const (&r)[2] = row;
  bool one = static_cast<const void*>(&m) == static_cast<const void*>(&member);
  bool two = static_cast<const void*>(&r) == static_cast<const void*>(&row);
  return one && two ? 0 : 1;
}
