// tidal-rules: UNEV-1
// tidal-expect: reject
// tidal-error-clang: with incomplete return type
// tidal-error-gcc: invalid use of incomplete type
// tidal-error-edg: returns incomplete type
struct Inc;
Inc make();
auto n = sizeof(make());
