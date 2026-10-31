// tidal-rules: TRY-2
// tidal-expect: run
struct B { virtual int id() { return 1; } };
struct D : B { int id() override { return 2; } };
int main() { try { throw D{}; } catch (B& b) { return b.id() == 2 ? 0 : 1; } return 3; }
