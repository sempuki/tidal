// tidal-rules: TRY-9 UNW-5
// tidal-expect: run-fail
// tidal-note: The destructor of a namespace-scope object throws after main returns. The function try block of main does not catch the exception, and the program ends.
struct G { ~G() noexcept(false) { throw 1; } };
G g;
int main() try { return 0; } catch (...) { return 0; }
