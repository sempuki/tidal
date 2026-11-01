// tidal-rules: ICS-31
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: ambiguous
// tidal-note: Q-0153. Both sequences add const to the element. ICS-31 counts only cv-qualifiers, so the first arguments tie and the second argument decides. Clang prefers the known bound, and the arguments then cross.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
int (*row)[3] = nullptr;
One take(const int (*)[3], long);
Two take(const int (*)[], int);
static_assert(same<decltype(take(row, 0)), Two>);
