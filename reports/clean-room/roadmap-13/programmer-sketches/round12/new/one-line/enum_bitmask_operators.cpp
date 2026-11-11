// Bit operators on a scoped enumeration.
enum class Perm : unsigned { r = 4, w = 2, x = 1 };
constexpr Perm operator|(Perm a, Perm b) { return static_cast<Perm>(static_cast<unsigned>(a) | static_cast<unsigned>(b)); }
constexpr bool has(Perm s, Perm p) { return (static_cast<unsigned>(s) & static_cast<unsigned>(p)) != 0; }
int main() { Perm p = Perm::r | Perm::x; return (has(p, Perm::x) && !has(p, Perm::w)) ? 0 : 1; }
