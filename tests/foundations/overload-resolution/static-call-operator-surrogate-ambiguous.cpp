// tidal-rules: OVL-36 OVL-25 OVL-6
// tidal-expect: reject
// tidal-error-clang: call to object of type .Wide. is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one
// tidal-note: The static operator wins on the object, and the surrogate wins on the int argument.
constexpr int other(int v) { return v + 10; }
struct Wide {
  static constexpr int operator()(long v) { return static_cast<int>(v) + 1; }
  using Pointer = int (*)(int);
  constexpr operator Pointer() const { return &other; }
};
int use() { Wide w; return w(1); }
