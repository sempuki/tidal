// tidal-rules: UNEV-1
// tidal-expect: reject
// tidal-error-clang: member access into incomplete type
// tidal-error-gcc: invalid use of incomplete type
// tidal-error-edg: returns incomplete type
// tidal-note: Gap review 2026-10-31.
struct Inc;
Inc make();
using T = decltype(make().x);
