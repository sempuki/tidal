// tidal-rules: REF-11
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: would use explicit constructor
// tidal-error-edg: copy-list-initialization cannot use a constructor
// tidal-note: Q-0150. Clang initializes the temporary as if the braces named the class.
struct Meters {
  explicit Meters(int v) : value(v) {}
  int value;
};
const Meters& length{3};
