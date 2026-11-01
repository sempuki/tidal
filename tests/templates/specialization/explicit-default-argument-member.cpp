// tidal-rules: TSP-8
// tidal-expect: reject
// tidal-error-clang: default argument not permitted on an explicit specialization
// tidal-error-gcc: default argument specified in explicit specialization
// tidal-error-edg: default argument is not allowed
// tidal-note: An explicit specialization of a member function of Box<char>, which the primary template gives.
template <class T> struct Box { int get(int); };
template <> int Box<char>::get(int = 1) { return 0; }
