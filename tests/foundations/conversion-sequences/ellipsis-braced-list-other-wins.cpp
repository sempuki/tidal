// tidal-rules: ICS-6
// tidal-expect: accept
// tidal-note: The braced list has no sequence to the ellipsis of show(...), so show(int, ...) is the only viable function. The list {1} meets its int parameter.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
long show(...);
int show(int, ...);
static_assert(same<decltype(show({1})), int>);
