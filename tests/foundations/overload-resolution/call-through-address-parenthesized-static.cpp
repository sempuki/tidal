// tidal-rules: OVL-44 OVL-21
// tidal-expect: reject
// tidal-error-clang: cannot create a non-constant pointer to member function
// tidal-error-gcc: address of overloaded function with no contextual type information
// tidal-error-edg: .
// tidal-note: The set holds the non-static f() & and the static f(int). The parenthesized qualified name cannot form the address of the non-static member, so the call is not valid although the static member would win. EDG stops with an internal error.
struct Item {
  int f() &;
  static int f(int = 0);
  int use() { return (&(Item::f))(1); }
};
