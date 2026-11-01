// tidal-rules: PACK-12
// tidal-expect: accept
// tidal-note: The expansion Ts... comes before int, so the list gives Ts no elements. The explicit argument gives them.
template <class... Ts> struct List {};
template <class... Ts> void take(List<Ts..., int>) {}
void use() { take<char>(List<char, int>{}); }
