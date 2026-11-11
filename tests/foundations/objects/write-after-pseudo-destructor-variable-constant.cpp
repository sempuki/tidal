// tidal-rules: OBJ-2
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: assignment to object outside its lifetime
// tidal-error-gcc: modification of .x. outside its lifetime
// tidal-note: RV9-9. A pseudo-destructor call ends the variable x, and the assignment then writes x outside its lifetime. The write is undefined, and constant evaluation rejects it. Clang and GCC reject. EDG accepts (Q-0447).
using Int = int;
constexpr int refill() {
  int x = 1;
  x.~Int();
  x = 3;
  return 1;
}
static_assert(refill() == 1);
