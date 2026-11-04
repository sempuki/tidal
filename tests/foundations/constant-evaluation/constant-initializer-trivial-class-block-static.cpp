// tidal-rules: CE-24 CE-21 CINIT-1
// tidal-expect: reject
// tidal-expect-gcc: run
// tidal-expect-edg: run
// tidal-error-clang: does not have a constant initializer
// tidal-note: Q-0192. A static variable in a block follows the namespace-scope case. GCC and EDG accept the zero.
struct Pair { int first; int second; };
int read() {
  static constinit Pair pair;
  return pair.first;
}
int main() { return read(); }
