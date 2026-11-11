// tidal-rules: CE-21
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-error-clang: must be initialized by a constant expression
// tidal-error-gcc: is not a constant expression
// tidal-error-edg: initialization is not constant
// tidal-note: Ledger row X25-7. The constructor ends the element items[1] with an explicit destructor call, so the element has no value, and the constant is not valid. Every newest compiler rejects the constant. GCC 14 accepts it (Q-0447).
using Int = int;
struct Row {
  int items[2];
  constexpr Row() : items{1, 2} { items[1].~Int(); }
};
constexpr Row row;
