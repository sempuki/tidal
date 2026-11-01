// tidal-rules: TPL-30 TPL-35
// tidal-expect: run
// tidal-note: After a pack, one parameter is found from the function parameter, and another has a default.
template <class... Front, class Back> int deduced(Back) { return sizeof...(Front); }
template <class... Front, class Back = int> int defaulted() { return sizeof...(Front); }
template <class... Front, class... Back> int packs(Back...) { return sizeof...(Back); }
int main() { return deduced<char, long>(1) == 2 && defaulted<char, long>() == 2 && packs(1, 2) == 2 ? 0 : 1; }
