// tidal-rules: LSP-4 LNK-5
// tidal-expect: run
// tidal-sources: c-name-unnamed-namespace-other-unit.tu.cpp
// tidal-note: Q-0369. The other unit defines the C function in an unnamed namespace, and this unit calls it.
extern "C" int shared_code();
int main() { return shared_code() == 3 ? 0 : 1; }
