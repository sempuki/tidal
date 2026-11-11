// tidal-rules: OBJ-2
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: assignment to object outside its lifetime
// tidal-note: RV9-9. A pseudo-destructor call ends the element v[1], and the assignment then writes the element outside its lifetime. An assignment creates no new object, so the write is undefined, and constant evaluation rejects it. Clang rejects. GCC and EDG accept and read the assignment as a new value (Q-0447).
using Int = int;
struct Row { int v[2]; };
constexpr int refill() {
  Row r{{1, 2}};
  r.v[1].~Int();
  r.v[1] = 3;
  return r.v[0];
}
static_assert(refill() == 1);
