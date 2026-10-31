// tidal-rules: OVL-36 OVL-3
// tidal-expect: reject
// tidal-error-clang: call to .reset. is ambiguous
// tidal-error-gcc: call of overloaded .reset\(\). is ambiguous
// tidal-error-edg: more than one instance of overloaded function .*reset.* matches the argument list
// tidal-note: Both members are candidates with no object. Neither is better, because the default argument takes no part (OVL-25).
struct Item {
  static void reset(int = 0);
  void reset();
  static void clear() { reset(); }
};
