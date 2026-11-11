// Bit flags on a scoped enumeration with operator| and operator& written for it.
enum class Perm : unsigned { none = 0, read = 1, write = 2, exec = 4 };
constexpr Perm operator|(Perm a, Perm b) { return static_cast<Perm>(static_cast<unsigned>(a) | static_cast<unsigned>(b)); }
constexpr Perm operator&(Perm a, Perm b) { return static_cast<Perm>(static_cast<unsigned>(a) & static_cast<unsigned>(b)); }
constexpr bool has(Perm set, Perm p) { return (set & p) != Perm::none; }
int main() {
  constexpr Perm rw = Perm::read | Perm::write;
  static_assert(has(rw, Perm::write) && !has(rw, Perm::exec));
  Perm p = Perm::none;
  p = p | Perm::exec;
  return has(p, Perm::exec) && !has(p, Perm::read) ? 0 : 1;
}
