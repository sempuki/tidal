// tidal-rules: OVL-3
// tidal-expect: reject
// tidal-error-clang: call to non-static member function without an object argument
// tidal-error-gcc: cannot call member function .void Item::reset\(\). without object
// tidal-error-edg: a nonstatic member reference must be relative to a specific object
// tidal-note: The static member function has no current object. Resolution still runs and chooses the non-static reset().
struct Item {
  void reset();
  static void reset(int);
  static void clear() { reset(); }
};
