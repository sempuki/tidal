// tidal-rules: OBJ-2
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: assignment to object outside its lifetime
// tidal-note: RV10-R-3. The pseudo-destructor call ends the element v[1], and the array member v stays active. The assignment names an element of the member, so it creates no new object and writes the element outside its lifetime. Clang rejects. GCC and EDG accept and read the assignment as a new value (Q-0447).
using Int = int;
union Cells { int v[2]; float ratio; };
constexpr int refill() {
  Cells c{{1, 2}};
  c.v[1].~Int();
  c.v[1] = 3;
  return c.v[1];
}
static_assert(refill() == 3);
