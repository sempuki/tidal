// A direct throw in a noexcept function: UNW-8 warning, then termination.
void f() noexcept { throw 1; }
int main() { try { f(); } catch (...) { return 0; } return 3; }
