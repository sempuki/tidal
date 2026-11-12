// tidal-rules: NAM-27 NAM-38 OVL-7
// tidal-expect: run
// tidal-note: Roadmap round 15, programmer reduction adl_operator_scoped_enum_own_namespace.cpp (from the tries of flags.cpp). The operator expressions are outside perm, and ordinary lookup from the use finds no operator| or operator&. The scoped enumeration Access makes perm an associated namespace (NAM-27), so argument-dependent lookup finds the two operator functions (NAM-38, OVL-7).
namespace perm {
enum class Access { none = 0, read = 1, write = 2 };
constexpr Access operator|(Access x, Access y) { return Access(int(x) | int(y)); }
constexpr bool operator&(Access x, Access y) { return (int(x) & int(y)) != 0; }
}
constexpr perm::Access both = perm::Access::read | perm::Access::write;
static_assert(both & perm::Access::write);
static_assert(!(perm::Access::read & perm::Access::write));
int main() {
  perm::Access mode = perm::Access::none | perm::Access::read;
  return (mode & perm::Access::read) && !(mode & perm::Access::write) ? 0 : 1;
}
