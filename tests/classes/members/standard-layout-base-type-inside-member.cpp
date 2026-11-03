// tidal-rules: CLS-32 CLS-10
// tidal-expect: accept
// tidal-note: Q-0341. The first member of Outer begins with a Unit, and Outer has a Unit base. The two Unit objects need two addresses, so CLS-32 makes Outer not standard-layout.
template <class A, class B> constexpr bool same_address(const A& a, const B& b) { return static_cast<const void*>(&a) == static_cast<const void*>(&b); }
struct Unit {};
struct Inner { Unit head; int value; };
struct Outer : Unit { Inner inner; };
constexpr Outer outer{};
static_assert(!same_address(static_cast<const Unit&>(outer), outer.inner.head));
