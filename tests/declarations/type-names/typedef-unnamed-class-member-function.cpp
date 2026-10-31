// tidal-rules: TYN-3
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-note: Q-0067. GCC 14 accepts it. GCC 16, Clang, and EDG reject it.
typedef struct { int v; int get() const { return v; } } Box;
int main() { Box b{4}; return b.get() == 4 ? 0 : 1; }
