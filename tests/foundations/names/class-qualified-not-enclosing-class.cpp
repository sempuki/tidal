// tidal-rules: NAM-18
// tidal-expect: reject
// tidal-error-clang: no member named .value. in .outer::inner.
// tidal-error-gcc: value. is not a member of .outer::inner
// tidal-error-edg: class .outer::inner. has no member .value.
struct outer { static constexpr int value = 3; struct inner { }; };
int read() { return outer::inner::value; }
