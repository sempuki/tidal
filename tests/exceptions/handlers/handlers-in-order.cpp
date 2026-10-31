// tidal-rules: TRY-1
// tidal-expect: run
struct B { int v; };
struct D : B {};
int main() { try { throw D{5}; } catch (B& b) { return b.v == 5 ? 0 : 1; } catch (D&) { return 2; } return 3; }
