// tidal-rules: ICS-8 DEL-1
// tidal-expect: reject
// tidal-error-clang: invokes a deleted function
// tidal-error-gcc: use of deleted function
// tidal-error-edg: it is a deleted function
// tidal-note: The deleted conversion function still forms a user-defined sequence, which beats the ellipsis. take(int) is chosen.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Source { operator int() = delete; };
One take(int);
Two take(...);
auto r = take(Source{});
