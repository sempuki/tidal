// tidal-rules: THR-9
// tidal-expect: run-fail
void f() noexcept { throw 1; }
int main() { try { f(); } catch (...) { return 0; } return 0; }
