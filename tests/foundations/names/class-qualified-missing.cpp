// tidal-rules: NAM-18
// tidal-expect: reject
// tidal-error-clang: no member named .value. in .box.
// tidal-error-gcc: value. is not a member of .box
// tidal-error-edg: class .box. has no member .value.
struct box { };
int read() { return box::value; }
