// tidal-rules: ICS-11 ICS-46
// tidal-expect: accept
// tidal-note: Decision 0059, item RV5, point RV5-B-6. An int* prvalue binds int* const& and const int* const& directly after it is materialized, so ICS-46 prefers the reference with fewer qualifiers below the top level, as for an int* lvalue.
constexpr int* make() { return nullptr; }
constexpr int pick(int* const&) { return 1; }
constexpr int pick(const int* const&) { return 2; }
static_assert(pick(make()) == 1);
