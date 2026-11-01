// tidal-rules: LSP-1
// tidal-expect: run
extern "C" {
using callback = int(int);             // a C function type
int apply(callback* f, int v) { return f(v); }
int twice(int v) { return 2 * v; }     // C language linkage
void run(int (*g)(int), int* out) { *out = g(5); }  // the parameter type has C language linkage too
}
int main() {
  int seen = 0;
  run(twice, &seen);
  return apply(twice, 2) == 4 && seen == 10 ? 0 : 1;
}
