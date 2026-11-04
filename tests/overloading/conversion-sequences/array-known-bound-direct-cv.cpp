// tidal-rules: ICS-46
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: call to .take. is ambiguous
// tidal-note: Q-0188. Both references bind directly to the lvalue row. int[2] converts to const int[] by a qualification adjustment, so ICS-46 prefers int (&)[2]. Clang finds a tie.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
int row[2];
One take(int (&)[2]);
Two take(const int (&)[]);
static_assert(same<decltype(take(row)), One>);
