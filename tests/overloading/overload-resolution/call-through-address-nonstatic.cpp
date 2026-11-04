// tidal-rules: OVL-3 OVL-21
// tidal-expect: reject
// tidal-error-clang: no matching function for call to .reset.|reference to overloaded function could not be resolved
// tidal-error-gcc: address of overloaded function with no contextual type information
// tidal-error-edg: .
// tidal-note: Through the address of the set there is no implied object argument, so the non-static reset() cannot be called.
struct Item {
  void reset();
  static void reset(int);
  void use() { (&Item::reset)(); }
};
