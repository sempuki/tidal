// tidal-rules: PFX-17
// tidal-expect: accept
int twice(int v) { return 2 * v; }
using Twice = int (*)(int);
struct Handle {
  operator Twice() const { return twice; }
};
int run(Handle h) { return h(4); }
