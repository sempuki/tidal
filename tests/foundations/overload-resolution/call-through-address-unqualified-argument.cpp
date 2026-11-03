// tidal-rules: OVL-21
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-expect-edg: reject
// tidal-error-gcc: address of overloaded function with no contextual type information
// tidal-error-edg: .
// tidal-note: Q-0161. The set of call-through-address-unqualified.cpp, called with an argument. &f resolves the set as a call by name (OVL-21), and the static member wins. GCC rejects the call, and EDG stops with an internal error.
struct Item {
  int f() &;
  static int f(int = 0);
  int use() { return (&f)(1); }
};
