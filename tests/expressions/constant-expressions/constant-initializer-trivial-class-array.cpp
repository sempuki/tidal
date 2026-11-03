// tidal-rules: CE-24 CE-21 CINIT-1
// tidal-expect: reject
// tidal-expect-gcc: run
// tidal-expect-edg: run
// tidal-error-clang: does not have a constant initializer
// tidal-note: Q-0192. Each element runs the default constructor of Pair. GCC and EDG accept the zero.
struct Pair { int first; int second; };
constinit Pair pairs[2];
int main() { return pairs[1].first; }
