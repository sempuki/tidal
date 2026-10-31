// tidal-rules: TRY-10
// tidal-expect: run
struct S { S() try { throw 1; } catch (int) { throw 2; } };
int main() { try { S s; } catch (int v) { return v == 2 ? 0 : 1; } return 2; }
