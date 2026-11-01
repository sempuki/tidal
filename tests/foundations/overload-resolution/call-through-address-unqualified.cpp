// tidal-rules: OVL-44 OVL-21
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-clang-18: reject
// tidal-expect-edg: accept
// tidal-error-clang-18: call to .f. is ambiguous
// tidal-error-gcc: address of overloaded function with no contextual type information
// tidal-note: Q-0161. &f with an unqualified name cannot form the address of the non-static member f() &, so the call is not valid. Clang 22 and EDG call the static member.
struct Item {
  int f() &;
  static int f(int = 0);
  int use() { return (&f)(); }
};
