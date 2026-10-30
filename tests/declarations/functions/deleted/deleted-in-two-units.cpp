// tidal-rules: DEL-5
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
// tidal-sources: deleted-in-two-units.tu.cpp
void banned(int) = delete;
int other();
int main() { return other(); }
