// tidal-rules: TYN-7
// tidal-expect: reject
struct Outer { struct Inner; };
struct Inner2 {};
struct Outer::Missing { int v; };
