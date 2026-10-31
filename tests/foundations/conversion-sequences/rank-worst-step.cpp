// tidal-rules: ICS-2
// tidal-expect: accept
// tidal-note: Array to pointer and const added are exact-match steps; the pointer to void step has conversion rank. short to int is a promotion and short to double a conversion.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
int values[2];
One take(const int*);
Two take(const void*);
static_assert(same<decltype(take(values)), One>);
short small = 1;
One widen(int);
Two widen(double);
static_assert(same<decltype(widen(small)), One>);
