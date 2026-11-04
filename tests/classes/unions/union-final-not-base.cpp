// tidal-rules: UNI-2 DER-1
// tidal-expect: reject
// tidal-error-clang: unions cannot be base classes
// tidal-error-gcc: fails to be a struct or class type
// tidal-error-edg: cannot not derive from a union
// tidal-note: A union is never a base, with or without final.
union Number final { int whole; };
struct Wider : Number { };
