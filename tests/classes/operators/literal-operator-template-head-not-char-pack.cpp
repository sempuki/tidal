// tidal-rules: OPR-10
// tidal-expect: reject
// tidal-error: literal operator
// tidal-note: Q-0435. A literal operator template whose head is a char followed by a pack of char has a form that OPR-8 does not give. Every compiler rejects the declaration.
template <char First, char... Rest> constexpr char operator""_lead() { return First; }
int main() { return 0; }
