// tidal-rules: ICS-31
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-expect-edg: reject
// tidal-error-gcc: ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: Q-0153. Both sequences add const to the element, and the second also drops the bound. The first result converts to the second by a qualification adjustment, so ICS-31 prefers the known bound. GCC and EDG find a tie.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
int (*row)[3] = nullptr;
One take(const int (*)[3]);
Two take(const int (*)[]);
static_assert(same<decltype(take(row)), One>);
