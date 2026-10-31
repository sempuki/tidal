// tidal-rules: BF-1
// tidal-expect: run
struct Flags { unsigned a : 1; unsigned b : 3; int c : 4; };
int main() { Flags f{}; f.a = 1; f.b = 5; f.c = -3; return f.a == 1 && f.b == 5 && f.c == -3 ? 0 : 1; }
