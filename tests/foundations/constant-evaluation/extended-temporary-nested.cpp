// tidal-rules: CE-14
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: temporary with limited
// tidal-note: Q-0193
struct Holder { const int& n; };
constexpr const Holder& held = Holder{4};
