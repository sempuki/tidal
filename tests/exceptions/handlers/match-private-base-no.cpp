// tidal-rules: TRY-2
// tidal-expect: run
struct B {}; struct D : private B {};
int main() { try { throw D{}; } catch (B&) { return 1; } catch (D&) { return 0; } return 2; }
