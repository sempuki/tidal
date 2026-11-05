struct X { template <class T> constexpr bool operator==(T* const&) const { return true; } };
template <class T> constexpr bool operator==(const X&, const T&) { return false; }
constexpr X x;
constexpr int* p = nullptr;
static_assert(x == p);
