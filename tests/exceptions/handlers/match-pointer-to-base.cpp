// tidal-rules: TRY-2
// tidal-expect: run
struct B {}; struct D : B {};
D obj;
int main() { try { throw &obj; } catch (B* p) { return p == &obj ? 0 : 1; } return 2; }
