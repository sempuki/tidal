// tidal-rules: OVL-44 OVL-21
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: cannot create a non-constant pointer to member function
// tidal-error-gcc: address of overloaded function with no contextual type information
// tidal-note: Q-0161. The same set as call-through-address-parenthesized-static.cpp, called with no argument. EDG calls the static member.
struct Item {
  int f() &;
  static int f(int = 0);
  int use() { return (&(Item::f))(); }
};
