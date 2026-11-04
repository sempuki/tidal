// tidal-rules: CMP-14
// tidal-expect: accept
// tidal-flags: -Werror -Wparentheses
struct Result { bool v; };
struct Level { int v; };
Result operator<(Level a, Level b) { return {a.v < b.v}; }
bool operator<(Result r, Level c) { return r.v && 0 < c.v; }
bool f(Level a, Level b, Level c) { return a < b < c; }   // the inner < gives a class
