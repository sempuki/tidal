// tidal-rules: OBJ-2 UNI-5 UNI-13
// tidal-expect: accept
// tidal-note: RV10-12. A destructor call ends a union member of class type with a trivial default constructor. An assignment to a scalar of that member, through the chain of member accesses, makes the member active with a new object, so the write and the read are valid in constant evaluation. Every compiler accepts.
struct Point { int x; };
union Slot { Point p; int i; };
constexpr int refill() {
  Slot s{.p = {1}};
  s.p.~Point();
  s.p.x = 5;
  return s.p.x;
}
static_assert(refill() == 5);
