// tidal-rules: OPR-8 OPR-10 PRIM-2
// tidal-expect: run
// tidal-note: Roadmap round 12, programmer sketch bigint_second_try.cpp, the literal _big. Q-0435 and Q-0436. A literal operator template with a pack of char gets the 30 digits of the literal as template arguments, so no integer type needs to hold the value. The fold sums the digits. Every compiler accepts.
template <char... Cs> constexpr int operator""_digitsum() { return (0 + ... + (Cs - '0')); }
static_assert(123456789012345678901234567890_digitsum == 135);
int main() { return 900000000000000000000000000001_digitsum == 10 ? 0 : 1; }
