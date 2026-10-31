// tidal-rules: ACC-5
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct Reader;
using Alias = Reader;
class C { int secret = 2; friend Reader; friend Alias; friend int; };
struct Reader { int get(const C& c) { return c.secret; } };
int main() { return Reader{}.get(C{}) == 2 ? 0 : 1; }
