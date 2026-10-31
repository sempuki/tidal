// tidal-rules: TRY-10
// tidal-expect: run
struct S { ~S() noexcept(false) try { throw 1; } catch (int) { } };
int main() { try { S s; } catch (int v) { return v == 1 ? 0 : 1; } return 2; }
