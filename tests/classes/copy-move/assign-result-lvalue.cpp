// tidal-rules: CPY-3
// tidal-expect: run
struct S { int v = 0; };
int main() { S a, b, c; c.v = 4; a = b = c; S& r = (a = c); return a.v == 4 && &r == &a ? 0 : 1; }
