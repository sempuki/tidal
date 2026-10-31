// tidal-rules: TRY-2
// tidal-expect: run
struct A {}; struct B : A {}; struct C : A {}; struct D : B, C {};
int main() { try { throw D{}; } catch (A&) { return 1; } catch (D&) { return 0; } return 2; }
