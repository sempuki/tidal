struct X { template <class T> constexpr bool operator==(T* const&) const { return true; } };
template <class T> constexpr bool operator==(const T&, const X&) { return false; }
constexpr X x;
constexpr int* p = nullptr;
static_assert(p == x);
