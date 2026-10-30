// tidal-rules: CMP-9
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: Gap review 2026-10-30. Q-0030. EDG accepts an ordered comparison of a pointer with nullptr, as with 0.
int x;
bool b = &x < nullptr;
