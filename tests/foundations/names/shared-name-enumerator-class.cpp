// tidal-rules: NAM-39
// tidal-expect: accept
// tidal-note: A class shares its name with an enumerator of an unscoped enumeration in the same namespace.
struct limit { int value; };
enum { limit = 3 };
