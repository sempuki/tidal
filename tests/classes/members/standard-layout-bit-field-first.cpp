// tidal-rules: CLS-30 CLS-10
// tidal-expect: accept
// tidal-note: A first member that is a bit-field has no address, so the address guarantee of CLS-30 does not reach the next member. The bit-field takes bytes, so the next member lies after the start of the object.
template <class A, class B> constexpr bool same_address(const A& a, const B& b) { return static_cast<const void*>(&a) == static_cast<const void*>(&b); }
struct Flags { int kind : 3; int count; };
constexpr Flags flags{};
static_assert(!same_address(flags, flags.count));
