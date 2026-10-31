// tidal-rules: ICS-31
// tidal-expect: accept
// tidal-note: Each sequence is one qualification adjustment, so neither is a subsequence of the other, and the ranks are equal.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
int* p = nullptr;
int** pp = nullptr;
One take(const int*);
Two take(const volatile int*);
static_assert(same<decltype(take(p)), One>);
One deep(const int* const*);
Two deep(const volatile int* const volatile*);
static_assert(same<decltype(deep(pp)), One>);
