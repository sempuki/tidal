// tidal-rules: CE-23 CX-1
// tidal-expect: reject
// tidal-error-clang: data member .inner. of non-literal type
// tidal-error-gcc: .Holder::inner. has non-literal type
// tidal-error-edg: cannot call non-constexpr function
struct Logged {
  int value;
  ~Logged() {}
};
struct Holder {
  Logged inner;
};
constexpr Holder holder{{1}};
