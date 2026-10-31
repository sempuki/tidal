// tidal-rules: CVF-4
// tidal-expect: run
int used = 0;
struct B { };
struct D : B { operator B() const { ++used; return B{}; } };
int main() { D d; B b = d; return used == 0 ? 0 : 1; }
