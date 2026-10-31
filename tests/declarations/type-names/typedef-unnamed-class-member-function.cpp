// tidal-rules: TYN-3
// tidal-expect: reject
typedef struct { int v; int get() const { return v; } } Box;
int main() { Box b{4}; return b.get() == 4 ? 0 : 1; }
