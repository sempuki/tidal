// tidal-rules: TRY-10
// tidal-expect: run
int trace = 0;
struct M { M(int v) { if (v == 0) throw 7; } };
struct S { M m; S(int v) try : m(v) { } catch (int e) { trace = e; } };
int main() { try { S s(0); } catch (int e) { return trace == 7 && e == 7 ? 0 : 1; } return 2; }
