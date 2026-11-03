// tidal-rules: OVL-21 OVL-36
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-clang-18: reject
// tidal-expect-edg: accept
// tidal-error-gcc: address of overloaded function with no contextual type information
// tidal-error-clang-18: call to .f. is ambiguous
// tidal-note: Q-0161. &f with an unqualified name and no parentheses resolves the set as a call by name with no implied object argument (OVL-21). The object parameter of f() & and the default argument of f(int = 0) take no part (OVL-23, OVL-25), so the two members tie and the call is ambiguous (OVL-36), as in no-object-static-default-ambiguous.cpp. Clang 18 reports the ambiguity, and GCC cannot resolve the set. Clang 22 and EDG choose the static member.
struct Item {
  int f() &;
  static int f(int = 0);
  int use() { return (&f)(); }
};
