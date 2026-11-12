// tidal-rules: CPT-26 NAM-27 NAM-29 NAM-30 OVL-7
// tidal-expect: run
// tidal-note: Roadmap round 15, programmer reduction adl_operator_through_using_declaration_requires.cpp (from the tries of flags.cpp). The requirement e | e needs argument-dependent lookup for a scoped enumeration operand (NAM-27, OVL-7). For perm::Access, a using declaration in perm names the operator template, so lookup finds it (NAM-29) and the requirement holds. For other::Mode, a using directive in other makes the template visible, which argument-dependent lookup ignores (NAM-30), and for Loose no operator exists. Those two requirements do not hold, and the program stays valid (CPT-26).
namespace bits {
template <class E> constexpr E operator|(E x, E y) { return E(int(x) | int(y)); }
}
namespace perm {
enum class Access { none = 0, read = 1, write = 2 };
using bits::operator|;
}
namespace other {
enum class Mode { quiet = 1, loud = 2 };
using namespace bits;
}
enum class Loose { a = 1 };
template <class E> concept combinable = requires(E e) { e | e; };
static_assert(combinable<perm::Access>);
static_assert(!combinable<other::Mode>);
static_assert(!combinable<Loose>);
int main() {
  bool found = combinable<perm::Access>;
  bool missed = combinable<other::Mode> || combinable<Loose>;
  return found && !missed ? 0 : 1;
}
