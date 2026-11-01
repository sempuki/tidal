// tidal-rules: TARG-14
// tidal-expect: accept
// tidal-note: The address of an array element and of a data member.
struct Pair { int first; int second; };
Pair pair{1, 2};
int values[3] = {1, 2, 3};
template <int* P> struct Ptr { static constexpr int* value = P; };
static_assert(Ptr<&pair.second>::value == &pair.second);
static_assert(Ptr<&values[1]>::value == &values[1]);
