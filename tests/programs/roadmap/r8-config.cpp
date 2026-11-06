// tidal-rules: ENUM-20 CLS-13 LIST-14 DCMP-1 DCMP-4 SB-5 CINIT-1
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-note: Q-0386. EDG checks every ordering class for the defaulted <=>, and the program declares only the classes it uses.
// tidal-note: Roadmap round 8, programmer sketch config.cpp (reports/clean-room/roadmap-8/programmer.md, section 1.4). The rules are the ones the report names for this program and its reductions, and the rules of the main constructs it uses.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Settings for a small service: an enum class used as a bit mask with
// constexpr operators, using enum in a switch, a Maybe<T> with a
// deduced object parameter on value(), a record with designated
// initializers, defaulted == and <=>, the tuple protocol for structured
// bindings, a sort on <=>, constinit, and if with an initializer.
namespace std {
using size_t = decltype(sizeof 0);
struct strong_ordering {
  signed char v;
  static const strong_ordering less, equal, equivalent, greater;
  friend constexpr bool operator==(strong_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(strong_ordering a, int) { return a.v < 0; }
  friend constexpr bool operator>(strong_ordering a, int) { return a.v > 0; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
template <class T> struct tuple_size;
template <size_t I, class T> struct tuple_element;
}

namespace app::config {

enum class Flag : unsigned { none = 0, tls = 1, gzip = 2, cache = 4, debug = 8 };

constexpr auto to_underlying(Flag f) noexcept { return static_cast<unsigned>(f); }
constexpr Flag operator|(Flag a, Flag b) noexcept { return Flag(to_underlying(a) | to_underlying(b)); }
constexpr Flag operator&(Flag a, Flag b) noexcept { return Flag(to_underlying(a) & to_underlying(b)); }
constexpr Flag& operator|=(Flag& a, Flag b) noexcept { return a = a | b; }
constexpr bool has(Flag set, Flag f) noexcept { return (set & f) != Flag::none; }

constexpr const char* name(Flag f) {
  switch (f) {
    using enum Flag;
    case none: return "none";
    case tls: return "tls";
    case gzip: return "gzip";
    case cache: return "cache";
    case debug: return "debug";
  }
  return "mixed";
}

template <class T>
class Maybe {
  bool has_ = false;
  union { T value_; };
public:
  constexpr Maybe() {}
  constexpr Maybe(T v) : has_(true), value_(v) {}
  constexpr explicit operator bool() const { return has_; }
  template <class Self> constexpr auto&& value(this Self&& self) {
    return static_cast<Self&&>(self).value_;
  }
  constexpr T value_or(T fallback) const { return has_ ? value_ : fallback; }
};

struct Endpoint {
  const char* host = "localhost";
  unsigned short port = 80;
  Flag flags = Flag::none;
  int priority = 0;
  friend constexpr bool operator==(const Endpoint&, const Endpoint&) = default;
};

struct Version {
  int major, minor, patch;
  constexpr auto operator<=>(const Version&) const = default;
  template <std::size_t I> constexpr int get() const {
    if constexpr (I == 0) return major;
    else if constexpr (I == 1) return minor;
    else return patch;
  }
};

}  // namespace app::config

template <> struct std::tuple_size<app::config::Version> { static constexpr std::size_t value = 3; };
template <std::size_t I> struct std::tuple_element<I, app::config::Version> { using type = int; };

namespace app::config {

template <class T, std::size_t N>
constexpr void sort(T (&a)[N]) {
  for (std::size_t i = 1; i < N; ++i)
    for (std::size_t j = i; j > 0 && a[j] < a[j - 1]; --j) {
      T t = a[j]; a[j] = a[j - 1]; a[j - 1] = t;
    }
}

constexpr Maybe<unsigned short> parse_port(const char* s) {
  unsigned v = 0;
  if (!*s) return {};
  for (; *s; ++s) {
    if (*s < '0' || *s > '9') return {};
    v = v * 10 + unsigned(*s - '0');
    if (v > 65535) return {};
  }
  return static_cast<unsigned short>(v);
}

constinit Endpoint fallback{.host = "backup", .flags = Flag::tls | Flag::cache};

}  // namespace app::config

using namespace app::config;

static_assert(parse_port("8080").value() == 8080);
static_assert(!parse_port("80a"));
static_assert(parse_port("70000").value_or(1) == 1);
static_assert(has(Flag::tls | Flag::gzip, Flag::gzip));
static_assert(Version{1, 2, 3} < Version{1, 10, 0});

int main() {
  Endpoint e{.port = 443, .flags = Flag::tls};
  e.flags |= Flag::gzip;
  if (e == Endpoint{.port = 443, .flags = Flag::tls}) return 1;
  if (!has(fallback.flags, Flag::cache) || fallback.port != 80) return 2;

  Version vs[] = {{2, 0, 1}, {1, 4, 0}, {1, 10, 2}, {1, 4, 0}};
  sort(vs);
  auto [maj, min, pat] = vs[3];
  if (maj != 2 || min != 0 || pat != 1) return 3;
  if (!(vs[1] == vs[0])) return 4;

  if (auto p = parse_port("9000"); p) {
    unsigned short& ref = p.value();
    ref += 1;
    if (p.value() != 9001) return 5;
  } else {
    return 6;
  }
  const char* n = name(Flag::gzip);
  return n[0] == 'g' && name(Flag::tls | Flag::debug)[0] == 'm' ? 0 : 7;
}
