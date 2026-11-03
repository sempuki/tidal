// Unpacking records with structured bindings.
// Built from Tidal: SB-1, SB-3, SB-4, SB-5, SB-6, SB-7, SB-10, SB-12,
// SB-17, LSUP-3, LSUP-4, RFOR (binding as loop variable), LAM (capture).

namespace std {
using size_t = decltype(sizeof 0);
template <class T> struct tuple_size;
template <size_t I, class T> struct tuple_element;
}

template <class A, class B> constexpr bool same_type = false;
template <class A> constexpr bool same_type<A, A> = true;

// Plain records: the names refer to the data members (SB-3).
struct Record {
  int id;
  const char* name;
  double score;
};

constexpr Record records[] = {{1, "ada", 9.5}, {2, "bob", 7.0}, {3, "cy", 8.0}};

constexpr int sum_ids() {
  int sum = 0;
  for (const auto& [id, name, score] : records) {
    if (name[0] != '\0' && score > 0.0) sum += id;
  }
  return sum;
}
static_assert(sum_ids() == 6);

// A class with private members uses the tuple protocol (SB-5, LSUP-3).
class Range {
  int lo_;
  int hi_;
public:
  constexpr Range(int lo, int hi) : lo_(lo), hi_(hi) {}
  template <std::size_t I> constexpr int& get() {
    if constexpr (I == 0) return lo_; else return hi_;
  }
  template <std::size_t I> constexpr const int& get() const {
    if constexpr (I == 0) return lo_; else return hi_;
  }
  constexpr int width() const { return hi_ - lo_; }
};

namespace std {
template <> struct tuple_size<Range> {
  static constexpr size_t value = 2;
};
template <size_t I> struct tuple_element<I, Range> {
  using type = int;
};
// `const auto [a, b] = r;` makes the hidden variable a `const Range`, and
// SB-5 asks for tuple_size of that type. Without these two, the binding
// falls back to the private members (SB-3) and is rejected.
template <> struct tuple_size<const Range> {
  static constexpr size_t value = 2;
};
template <size_t I> struct tuple_element<I, const Range> {
  using type = const int;
};
}

constexpr int widen() {
  Range r{2, 5};
  auto& [lo, hi] = r;          // names refer to r's members through get
  lo -= 1;
  hi += 1;
  return r.width();
}
static_assert(widen() == 5);

constexpr int copy_then_change() {
  Range r{2, 5};
  auto [lo, hi] = r;           // the hidden variable is a copy (SB-1)
  lo = 100;
  return r.width() + hi;
}
static_assert(copy_then_change() == 8);

constexpr int read_const() {
  const auto [lo, hi] = Range{3, 4};
  static_assert(same_type<decltype(lo), const int>);
  return lo + hi;
}
static_assert(read_const() == 7);

// decltype of each name (SB-4, SB-5).
struct Mixed {
  int count;
  const double ratio;
  int& alias;
};
int target = 0;
Mixed mixed{1, 0.5, target};
auto& [m_count, m_ratio, m_alias] = mixed;
static_assert(same_type<decltype(m_count), int>);
static_assert(same_type<decltype(m_ratio), const double>);
static_assert(same_type<decltype(m_alias), int&>);
const auto [c_count, c_ratio, c_alias] = mixed;
static_assert(same_type<decltype(c_count), const int>);
static_assert(same_type<decltype(c_alias), int&>);   // const is not added
Range global_range{0, 3};
auto [g_lo, g_hi] = global_range;
static_assert(same_type<decltype(g_lo), int>);

// A counter to watch the life of a temporary (SB-6).
struct Pair {
  static inline int live = 0;
  int first;
  int second;
  Pair(int a, int b) : first(a), second(b) { ++live; }
  Pair(const Pair& other) : first(other.first), second(other.second) { ++live; }
  ~Pair() { --live; }
};
Pair make_pair(int a, int b) { return Pair(a, b); }

struct Flags {
  unsigned ready : 1;
  unsigned level : 3;
};

int next_id() {
  static auto [base, step] = Range{100, 10};   // static binding (SB-7)
  base += step;
  return base;
}

int main() {
  int result = 0;

  // SB-1: copy versus reference to an array.
  int a[2] = {1, 2};
  auto [x, y] = a;
  x = 10;
  if (a[0] != 1 || y != 2) result |= 1;
  auto& [p, q] = a;
  p = 5;
  if (a[0] != 5 || q != 2) result |= 2;
  auto [c, d]{a};              // braced initializer (SB-7)
  if (c != 5 || d != 2) result |= 4;

  // SB-12: each name is an lvalue, so its address can be taken.
  int* px = &x;
  if (*px != 10) result |= 8;

  // SB-6: the temporary lives as long as the names.
  {
    const auto& [first, second] = make_pair(3, 4);
    if (Pair::live != 1 || first + second != 7) result |= 16;
  }
  if (Pair::live != 0) result |= 32;

  // SB-17: a tuple-protocol binding to a temporary Range.
  auto&& [lo, hi] = Range{4, 9};
  if (hi - lo != 5) result |= 64;

  // SB-10: a binding to a bit-field can be assigned.
  Flags flags{0, 0};
  auto& [ready, level] = flags;
  ready = 1;
  level = 5;
  if (flags.ready != 1 || flags.level != 5) result |= 128;

  // A lambda can capture a binding.
  auto [id, name, score] = records[1];
  auto describe = [id, name] { return id * 10 + (name[0] == 'b' ? 1 : 0); };
  if (describe() != 21 || score != 7.0) result |= 256;

  // Names through a reference member write to the referenced object.
  m_alias = 42;
  if (target != 42 || c_alias != 42) result |= 512;

  if (next_id() != 110 || next_id() != 120) result |= 1024;
  if (g_lo != 0 || g_hi != 3) result |= 2048;
  return result;
}
