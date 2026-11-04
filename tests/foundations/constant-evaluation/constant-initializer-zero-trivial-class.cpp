// tidal-rules: CE-24 CE-21 CINIT-1
// tidal-expect: reject
// tidal-expect-gcc: run
// tidal-expect-edg: run
// tidal-error-clang: does not have a constant initializer
// tidal-note: Q-0192. The default constructor runs and leaves both members without a value. GCC and EDG accept the zero.
struct Pair { int first; int second; };
constinit Pair pair;
int main() { return pair.first == 0 && pair.second == 0 ? 0 : 1; }
