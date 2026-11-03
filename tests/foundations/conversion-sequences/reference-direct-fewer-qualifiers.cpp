// tidal-rules: ICS-46
// tidal-expect: accept
// tidal-note: Both references bind directly to the int* lvalue p. int* converts to const int* by a qualification adjustment, so ICS-46 prefers int* const&. In the same way const int* const& wins over const volatile int* const&.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
int* p = nullptr;
One take(int* const&);
Two take(const int* const&);
static_assert(same<decltype(take(p)), One>);
One give(const int* const&);
Two give(const volatile int* const&);
static_assert(same<decltype(give(p)), One>);
