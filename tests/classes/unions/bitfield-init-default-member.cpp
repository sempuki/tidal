// tidal-rules: BF-7
// tidal-expect: run
struct S { unsigned x : 4 = 7; int y : 4 {2}; };
int main() { S s; return s.x == 7 && s.y == 2 ? 0 : 1; }
