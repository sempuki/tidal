// tidal-rules: TRY-10
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct M { ~M() { mark(1); } };
struct S { M m; S() try { throw 1; } catch (int) { mark(2); } };
int main() { try { S s; } catch (int) { mark(3); } return trace == 123 ? 0 : 1; }
