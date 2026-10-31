// tidal-rules: THR-10
// tidal-expect: run
constexpr int check(int x) { if (x < 0) throw 1; return x; }
int main() { try { return check(-1); } catch (int e) { return e == 1 ? 0 : 1; } }
