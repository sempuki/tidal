// tidal-rules: DEP-39
// tidal-expect: run
// tidal-note: The object t is type-dependent, so later is looked up at the instantiation, in Probe. Probe is declared after the template.
template <class T> int call(T t) { return t.later(); }
struct Probe { int later() const { return 4; } };
int main() { return call(Probe{}) == 4 ? 0 : 1; }
