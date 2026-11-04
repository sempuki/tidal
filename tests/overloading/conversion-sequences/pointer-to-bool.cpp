// tidal-rules: ICS-32
// tidal-expect: accept
// tidal-note: int* to bool and int* to void* both have conversion rank, and no other tie-breaker applies to bool or void.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Item { int x; };
int* p = nullptr;
One take(bool);
Two take(void*);
static_assert(same<decltype(take(p)), Two>);
One member(bool);
Two member(...);
static_assert(same<decltype(member(&Item::x)), One>);
