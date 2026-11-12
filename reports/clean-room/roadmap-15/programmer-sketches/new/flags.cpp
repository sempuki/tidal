// Flag sets over scoped enumerations. An enumeration opts in through a
// traits specialization that gives its underlying type and a table of
// names. Flags<E> holds a set, iterates over its members, prints itself
// into a fixed buffer, and parses "read|write" back. No standard header.

using size_t = decltype(sizeof 0);

namespace fl {

template <class E> struct traits;  // no definition: not a flag enum

template <class E>
concept flag_enum = requires {
  typename traits<E>::underlying;
  traits<E>::names[0];
  traits<E>::count;
};

template <class E>
using under = typename traits<E>::underlying;

constexpr bool equal(const char* a, size_t an, const char* b) {
  for (size_t i = 0; i < an; ++i)
    if (b[i] != a[i]) return false;
  return b[an] == '\0';
}

struct Buf {
  char text[64]{};
  size_t len = 0;
  constexpr void put(const char* s) {
    while (*s) text[len++] = *s++;
    text[len] = '\0';
  }
  constexpr bool operator==(const char* s) const { return equal(text, len, s); }
};

template <flag_enum E>
class Flags {
 public:
  using U = under<E>;

  constexpr Flags() = default;
  constexpr Flags(E e) : bits_(static_cast<U>(e)) {}

  constexpr bool test(E e) const { return (bits_ & static_cast<U>(e)) == static_cast<U>(e); }
  constexpr Flags& set(E e) {
    bits_ |= static_cast<U>(e);
    return *this;
  }
  constexpr Flags& reset(E e) {
    bits_ &= static_cast<U>(~static_cast<U>(e));
    return *this;
  }
  constexpr int count() const {
    int n = 0;
    for (U b = bits_; b; b &= b - 1) ++n;
    return n;
  }
  constexpr bool empty() const { return bits_ == 0; }
  constexpr U raw() const { return bits_; }

  friend constexpr Flags operator|(Flags a, Flags b) { return Flags(a.bits_ | b.bits_, 0); }
  friend constexpr bool operator==(Flags, Flags) = default;

  // Iterates over the single-bit members, lowest first.
  class iterator {
   public:
    constexpr iterator(U rest) : rest_(rest) {}
    constexpr E operator*() const { return static_cast<E>(rest_ & -rest_); }
    constexpr iterator& operator++() {
      rest_ &= rest_ - 1;
      return *this;
    }
    constexpr bool operator==(const iterator&) const = default;

   private:
    U rest_;
  };
  constexpr iterator begin() const { return iterator(bits_); }
  constexpr iterator end() const { return iterator(0); }

 private:
  constexpr Flags(U bits, int) : bits_(bits) {}
  U bits_ = 0;
};

template <flag_enum E>
constexpr const char* name_of(E e) {
  for (size_t i = 0; i < traits<E>::count; ++i)
    if (traits<E>::values[i] == e) return traits<E>::names[i];
  return "?";
}

template <flag_enum E>
constexpr Buf to_string(Flags<E> f) {
  Buf b;
  if (f.empty()) {
    b.put("none");
    return b;
  }
  bool first = true;
  for (E e : f) {
    if (!first) b.put("|");
    b.put(name_of(e));
    first = false;
  }
  return b;
}

template <class E>
struct Parsed {
  bool ok;
  Flags<E> value;
};

template <flag_enum E>
constexpr Parsed<E> parse(const char* s) {
  Flags<E> out;
  while (*s) {
    size_t n = 0;
    while (s[n] && s[n] != '|') ++n;
    bool found = false;
    for (size_t i = 0; i < traits<E>::count; ++i) {
      if (equal(s, n, traits<E>::names[i])) {
        out.set(traits<E>::values[i]);
        found = true;
      }
    }
    if (!found) return {false, {}};
    s += n;
    if (*s == '|') ++s;
  }
  return {true, out};
}

}  // namespace fl

// The operators on the enumerations themselves live in the global
// namespace, where unqualified lookup finds them from every namespace
// that declares no operator of its own.
template <fl::flag_enum E>
constexpr E operator|(E a, E b) {
  return static_cast<E>(static_cast<fl::under<E>>(a) | static_cast<fl::under<E>>(b));
}
template <fl::flag_enum E>
constexpr E operator&(E a, E b) {
  return static_cast<E>(static_cast<fl::under<E>>(a) & static_cast<fl::under<E>>(b));
}
template <fl::flag_enum E>
constexpr E operator~(E a) {
  return static_cast<E>(~static_cast<fl::under<E>>(a) & fl::traits<E>::mask);
}


namespace fs {

enum class Perm : unsigned char { none = 0, read = 1, write = 2, exec = 4 };
enum class Color { red, green };  // not a flag enum

}  // namespace fs

template <>
struct fl::traits<fs::Perm> {
  using underlying = unsigned char;
  static constexpr size_t count = 3;
  static constexpr fs::Perm values[] = {fs::Perm::read, fs::Perm::write, fs::Perm::exec};
  static constexpr const char* names[] = {"read", "write", "exec"};
  static constexpr unsigned char mask = 7;
};

namespace fs {

using fl::Flags;

constexpr const char* describe(Perm p) {
  using enum Perm;
  switch (p) {
    case none: return "-";
    case read: return "r";
    case write: return "w";
    case exec: return "x";
  }
  return "?";
}

constexpr bool can_run(Flags<Perm> f) { return f.test(Perm::read | Perm::exec); }

}  // namespace fs

using fs::Perm;
using fl::Flags;

template <class E>
concept has_or = requires(E a) { a | a; };
static_assert(has_or<Perm>);
static_assert(!has_or<fs::Color>);
static_assert(fl::flag_enum<Perm> && !fl::flag_enum<fs::Color>);

constexpr bool flags_test() {
  Flags<Perm> f = Perm::read | Perm::write;
  if (!f.test(Perm::read) || f.test(Perm::exec) || f.count() != 2) return false;
  if (!(fl::to_string(f) == "read|write")) return false;
  f.set(Perm::exec).reset(Perm::write);
  if (!fs::can_run(f) || !(fl::to_string(f) == "read|exec")) return false;
  if (!(fl::to_string(Flags<Perm>{}) == "none")) return false;
  if ((~Perm::read) != (Perm::write | Perm::exec)) return false;
  auto p = fl::parse<Perm>("exec|read");
  if (!p.ok || p.value != f) return false;
  if (fl::parse<Perm>("read|wrte").ok) return false;
  int seen = 0;
  for (Perm e : Flags<Perm>(Perm::read | Perm::write | Perm::exec)) seen = seen * 10 + static_cast<int>(e);
  if (seen != 124) return false;
  return fs::describe(Perm::write)[0] == 'w';
}
static_assert(flags_test());

int main() {
  if (!flags_test()) return 1;
  Flags<Perm> f;
  const char* words[] = {"read", "write"};
  for (const char* s : words) f = f | fl::parse<Perm>(s).value;
  return f.raw() == 3 ? 0 : 2;
}
