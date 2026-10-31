// tidal-rules: CEV-1
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
consteval int* make() { return new int(1); }
int main() { int* p = make(); delete p; return 0; }
