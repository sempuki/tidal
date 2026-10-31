// tidal-rules: CMP-9
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: invalid operands to binary expression .*nullptr_t
// tidal-error-gcc: ordered comparison of pointer with integer zero
// tidal-note: Gap review 2026-10-30. Q-0030. EDG accepts an ordered comparison of a pointer with nullptr, as with 0.
int x;
bool b = &x < nullptr;
