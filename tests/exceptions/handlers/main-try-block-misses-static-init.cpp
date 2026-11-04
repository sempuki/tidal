// tidal-rules: TRY-9 UNW-5
// tidal-expect: run-fail
// tidal-note: The constructor of a namespace-scope object throws before main starts. The function try block of main does not catch the exception, and the program ends.
struct G { G() { throw 1; } };
G g;
int main() try { return 0; } catch (...) { return 0; }
