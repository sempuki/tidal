// tidal-rules: ICS-26 ICS-31
// tidal-expect: accept
// tidal-note: ICS-31 gives the same answer here.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
int* p = nullptr;
int values[2];
One take(int*);
Two take(const int*);
static_assert(same<decltype(take(p)), One>);
static_assert(same<decltype(take(values)), One>);
