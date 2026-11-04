// tidal-rules: CE-14
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: temporary with limited
// tidal-note: Q-0193
struct Pair { int first; int second; };
constexpr const int& second = Pair{1, 2}.second;
constexpr const int* where = &second;
