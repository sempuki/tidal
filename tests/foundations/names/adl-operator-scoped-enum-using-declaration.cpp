// tidal-rules: NAM-27 NAM-29 OVL-7
// tidal-expect: run
// tidal-note: Roadmap round 15, programmer probes adl_operator_template_through_using_declaration.cpp and adl_operator_function_through_using_declaration.cpp (from the tries of flags.cpp). Each operator function is declared in bits, and a using declaration in the namespace of the scoped enumeration names it. The expressions are outside both namespaces. Argument-dependent lookup searches the namespace of the enumeration (NAM-27) and finds the functions that a using declaration there names (NAM-29), so the operator expression selects them (OVL-7).
namespace perm { enum class Access { none = 0, read = 1, write = 2 }; }
namespace bits {
template <class E> constexpr E operator|(E x, E y) { return E(int(x) | int(y)); }
constexpr bool operator&(perm::Access x, perm::Access y) { return (int(x) & int(y)) != 0; }
}
namespace perm {
using bits::operator|;
using bits::operator&;
}
static_assert(int(perm::Access::read | perm::Access::write) == 3);
static_assert((perm::Access::read | perm::Access::write) & perm::Access::write);
int main() {
  perm::Access mode = perm::Access::read | perm::Access::none;
  return (mode & perm::Access::read) && !(mode & perm::Access::write) ? 0 : 1;
}
