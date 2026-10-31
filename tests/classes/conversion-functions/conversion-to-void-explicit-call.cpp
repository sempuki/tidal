// tidal-rules: CVF-4
// tidal-expect: run
int used = 0;
struct S { operator void() const { ++used; } };
int main() { S s; s.operator void(); return used == 1 ? 0 : 1; }
