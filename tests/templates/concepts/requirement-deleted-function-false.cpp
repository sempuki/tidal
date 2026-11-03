// tidal-rules: CPT-26
// tidal-expect: accept
// tidal-note: Write report W5. A dependent requirement whose call selects a deleted function does not hold, and the program stays valid.
int twice(int v) { return 2 * v; }
int twice(double) = delete;
template <class T> concept Twiceable = requires(T v) { twice(v); };
static_assert(Twiceable<int>);
static_assert(!Twiceable<double>);
