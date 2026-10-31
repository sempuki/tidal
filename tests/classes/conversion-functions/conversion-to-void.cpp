// tidal-rules: CVF-4
// tidal-expect: run
int used = 0;
struct S { operator void() const { ++used; } };
int main() { S s; static_cast<void>(s); return used == 0 ? 0 : 1; }
