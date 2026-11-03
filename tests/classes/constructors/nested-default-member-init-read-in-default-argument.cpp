// tidal-rules: CTOR-18
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: default member initializer for .v. needed within definition of enclosing class
// tidal-error-gcc: default member initializer for .Outer::In::v. required before the end of its enclosing class
// tidal-note: Q-0083. Clang and GCC reject, EDG accepts. The default argument reads a member of a value-initialized nested object. The value needs the default member initializer before the enclosing class is complete.
struct Outer {
  struct In { int v = 1; };
  int n;
  Outer(int x = In{}.v) : n(x) {}
};
Outer o;
