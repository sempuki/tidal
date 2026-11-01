// tidal-rules: REF-6
// tidal-expect: accept
void plain(int) {}
void (&call)(int){plain};
void (&&other)(int){plain};
constexpr bool same = &call == &plain && &other == &plain;
static_assert(same);
