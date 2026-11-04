// tidal-rules: ICS-8 DEL-1
// tidal-expect: reject
// tidal-error-clang: invokes a deleted function
// tidal-error-gcc: use of deleted function
// tidal-error-edg: it is a deleted function
// tidal-note: The deleted constructor still forms a user-defined sequence, which beats the ellipsis. take(Gate) is chosen, and its conversion uses the deleted constructor.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Gate { Gate(int) = delete; };
One take(Gate);
Two take(...);
auto r = take(1);
