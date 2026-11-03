// tidal-rules: CLS-19 CLS-10
// tidal-expect: accept
// tidal-note: The base of Wrapper and its first member have one type, so they are two objects of one type and need two addresses. Wrapper is not standard-layout, and its first member does not lie at its address on the matrix targets.
template <class A, class B> constexpr bool same_address(const A& a, const B& b) { return static_cast<const void*>(&a) == static_cast<const void*>(&b); }
struct Unit {};
struct Wrapper : Unit { Unit head; int value; };
constexpr Wrapper wrapper{};
static_assert(!same_address(static_cast<const Unit&>(wrapper), wrapper.head));
