// tidal-rules: CE-24
// tidal-expect: run
// tidal-expect-clang: reject
// tidal-error-clang: does not have a constant initializer
// tidal-note: Q-0192
struct Pair { int first; int second; };
constinit Pair pair;
int main() { return pair.first == 0 && pair.second == 0 ? 0 : 1; }
