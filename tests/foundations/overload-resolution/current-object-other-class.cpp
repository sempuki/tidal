// tidal-rules: OVL-2 OVL-3
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-edg: accept
// tidal-error-gcc: cannot call member function .* without object
// tidal-note: Q-0159. Lookup in Inner::run finds the members of Outer. The current object is an Inner, which is not an Outer, so the call has no object. The non-static f(long) is an exact match and wins, then needs an object. Clang and EDG choose the static f(int).
struct Outer {
  static int f(int);
  int f(long);
  struct Inner {
    int run() { return f(1L); }
  };
};
