// tidal-rules: OBJ-2 UNI-5 UNI-13
// tidal-expect: accept
// tidal-note: RV10-12. A pseudo-destructor call ends a scalar member of a union that is a member of a struct. An assignment through the chain of member accesses makes the scalar member active with a new object, so the write and the read are valid in constant evaluation. Every compiler accepts.
using Int = int;
union Slot { int i; float f; };
struct Box { Slot slot; };
constexpr int refill() {
  Box b{.slot = {.i = 1}};
  b.slot.i.~Int();
  b.slot.i = 5;
  return b.slot.i;
}
static_assert(refill() == 5);
