// tidal-rules: CE-23 CX-1
// tidal-expect: reject
// tidal-error-clang: destructor is not constexpr
// tidal-error-gcc: does not have .constexpr. destructor
// tidal-error-edg: cannot call non-constexpr function
struct Logged {
  int value;
  ~Logged() {}
};
constexpr Logged logged{1};
