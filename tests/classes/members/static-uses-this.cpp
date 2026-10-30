// tidal-rules: CLS-7
// tidal-expect: reject
struct S {
  int v;
  static int get() { return this->v; }
};
