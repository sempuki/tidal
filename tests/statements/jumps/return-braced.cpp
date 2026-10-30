// tidal-rules: JMP-3
// tidal-expect: run
struct P { int a; int b; };
P make() { return {1, 2}; }
int main() { return make().b == 2 ? 0 : 1; }
