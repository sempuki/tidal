// tidal-rules: OVL-3 OVL-23
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: call to non-static member function without an object argument
// tidal-error-edg: nonstatic member reference must be relative to a specific object
// tidal-note: Q-0187. The call has no object, so the object parameter of put(int) && takes no part. put(int) && wins on the int argument and has no object. GCC gives the call an lvalue object, so put(int) && is not viable there and the static member wins.
struct Store {
  static void put(long);
  void put(int) &&;
  static void fill() { put(1); }
};
