// tidal-rules: CLS-19 CLS-10
// tidal-expect: accept
// tidal-note: Both bases of Pair contain a Unit base, so Pair has two Unit subobjects. They need two addresses, and Pair is not standard-layout.
template <class A, class B> constexpr bool same_address(const A& a, const B& b) { return static_cast<const void*>(&a) == static_cast<const void*>(&b); }
struct Unit {};
struct Left : Unit {};
struct Right : Unit {};
struct Pair : Left, Right { int value; };
constexpr Pair pair{};
static_assert(!same_address(static_cast<const Unit&>(static_cast<const Left&>(pair)), static_cast<const Unit&>(static_cast<const Right&>(pair))));
