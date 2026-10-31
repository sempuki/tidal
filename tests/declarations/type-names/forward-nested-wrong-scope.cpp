// tidal-rules: TYN-6
// tidal-expect: reject
struct Outer { struct Inner; };
struct Inner2 {};
struct Outer::Missing { int v; };
