// tidal-rules: TYN-7
// tidal-expect: reject
// tidal-error-clang: member access into incomplete type
// tidal-error-gcc: invalid use of incomplete type
// tidal-error-edg: incomplete type .Engine. is not allowed
class Engine;
int power(Engine* e) { return e->power; }
