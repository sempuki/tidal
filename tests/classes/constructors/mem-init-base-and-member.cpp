// tidal-rules: CTOR-5
// tidal-expect: run
struct B { int x; B(int v) : x(v) {} };
struct S : B { int y; S() : B(3), y(4) {} };
int main() { S s; return s.x + s.y == 7 ? 0 : 1; }
