// tidal-rules: ICS-19
// tidal-expect: accept
// tidal-note: No constructor of Split is best, so the sequence is the ambiguous sequence. It does not beat Even, and the second argument decides.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Split { Split(int, long); Split(long, int); };
struct Even { Even(int, int); };
One take(Split, long);
Two take(Even, int);
static_assert(same<decltype(take({1, 1}, 0)), Two>);
