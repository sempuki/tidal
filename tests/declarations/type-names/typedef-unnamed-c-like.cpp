// tidal-rules: TYN-3
// tidal-expect: run
typedef struct { int v; int w; } Pair;
int sum(Pair p) { return p.v + p.w; }
int main() { Pair p{1, 2}; return sum(p) == 3 ? 0 : 1; }
