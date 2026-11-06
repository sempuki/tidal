// tidal-rules: ICS-17
// tidal-expect: accept
// tidal-note: Decision 0059, item RV5, point RV5-B-7. A list of three elements has no sequence to a reference to an array of bound 2, so the overload for bound 3 is chosen. The condition follows the compilers.
constexpr int size_of(const int (&)[2]) { return 2; }
constexpr int size_of(const int (&)[3]) { return 3; }
static_assert(size_of({1, 2, 3}) == 3);
