// tidal-rules: TYN-7
// tidal-expect: reject
// tidal-error-clang: incomplete result type
// tidal-error-gcc: return type .* is incomplete
// tidal-error-edg: returns incomplete type
struct Inc;
Inc make() { while (true) {} }
