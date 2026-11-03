// tidal-rules: OVL-21
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-expect-clang-18: reject
// tidal-error-gcc: address of overloaded function with no contextual type information
// tidal-error-clang-18: call to .f. is ambiguous
// tidal-note: Q-0161. &f with an unqualified name and no parentheses resolves the set as a call by name with no implied object argument, as for &Item::f (OVL-21, Q-0143). The static member wins. GCC rejects the call, and Clang 18 finds it ambiguous.
struct Item {
  int f() &;
  static int f(int = 0);
  int use() { return (&f)(); }
};
