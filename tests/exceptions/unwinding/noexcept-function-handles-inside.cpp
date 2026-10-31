// tidal-rules: UNW-6
// tidal-expect: run
int safe() noexcept { try { throw 2; } catch (int v) { return v; } }
int main() { return safe() == 2 ? 0 : 1; }
