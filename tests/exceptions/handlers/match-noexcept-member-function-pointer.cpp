// tidal-rules: TRY-2
// tidal-expect: run
// tidal-note: A pointer to a noexcept member function matches a handler for a pointer to a member function without noexcept.
struct M { int f() noexcept { return 7; } };
int main() { try { throw &M::f; } catch (int (M::*p)()) { M m; return (m.*p)() == 7 ? 0 : 1; } catch (...) { return 2; } return 3; }
