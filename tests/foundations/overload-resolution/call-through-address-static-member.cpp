// tidal-rules: OVL-21
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-expect-edg: reject
// tidal-expect-clang-18: reject
// tidal-error-clang-18: reference to overloaded function could not be resolved
// tidal-error-gcc: address of overloaded function with no contextual type information
// tidal-error-edg: .
// tidal-note: Q-0143. The set holds a non-static and a static member. The static one wins and needs no object. GCC and Clang 18 reject the form, and EDG stops with an internal error.
struct Item {
  void reset();
  static void reset(int);
  void use() { (&Item::reset)(1); }
};
