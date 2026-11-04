// tidal-rules: UNW-5
// tidal-expect: run-fail
// tidal-note: The destructor of an object with static storage duration throws after main returns, and the program ends.
struct G { ~G() noexcept(false) { throw 1; } };
G g;
int main() { try { return 0; } catch (...) { return 0; } }
