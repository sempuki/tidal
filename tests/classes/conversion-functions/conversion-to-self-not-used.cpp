// tidal-rules: CVF-4
// tidal-expect: run
int used = 0;
struct S { operator S() const { ++used; return S{}; } };
int main() { S a; S b = a; return used == 0 ? 0 : 1; }
