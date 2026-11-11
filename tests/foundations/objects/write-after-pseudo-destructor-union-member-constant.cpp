// tidal-rules: OBJ-2 UNI-5 UNI-14
// tidal-expect: accept
// tidal-note: RV10-R-3. The pseudo-destructor call leaves the union with no active member. An assignment through the name of the scalar member then makes the member active with a new object, so the write and the read are valid in constant evaluation.
using Int = int;
union Cell { int count; float ratio; };
constexpr int refill() {
  Cell c{1};
  c.count.~Int();
  c.count = 3;
  return c.count;
}
static_assert(refill() == 3);
