// tidal-rules: CLS-9
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
struct Value {
  int kind;
  union { int i; float f; };
};
int get(Value v) { return v.i; }
