// tidal-rules: OBJ-2
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: assignment to object outside its lifetime
// tidal-note: F7-2. The pseudo-destructor call ends first, an int member of the struct member pair, and pair stays the active member. The assignment names a member of pair, a class member that is alive, so it creates no new object and writes first outside its lifetime. Clang rejects. GCC and EDG accept and read the assignment as a new value (Q-0447).
using Int = int;
struct Pair { int first; int second; };
union Store { Pair pair; float ratio; };
constexpr int refill() {
  Store s{.pair = {1, 2}};
  s.pair.first.~Int();
  s.pair.first = 3;
  return s.pair.first + s.pair.second;
}
static_assert(refill() == 5);
