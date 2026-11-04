// The language-support shapes from specification/foundations/language-support.md,
// gathered in one file and used together. No #include.
namespace std {
using size_t = decltype(sizeof 0);
using ptrdiff_t = decltype((int*)0 - (int*)0);
using nullptr_t = decltype(nullptr);
template <class E> class initializer_list {
  const E* first;
  size_t count;
  constexpr initializer_list(const E* f, size_t n) : first(f), count(n) {}
public:
  constexpr const E* begin() const { return first; }
  constexpr const E* end() const { return first + count; }
  constexpr size_t size() const { return count; }
};
template <class E> struct tuple_size;
template <size_t I, class E> struct tuple_element;
class partial_ordering {
  signed char v;
public:
  constexpr explicit partial_ordering(signed char x) : v(x) {}
  static const partial_ordering less, equivalent, greater, unordered;
  friend constexpr bool operator==(partial_ordering, partial_ordering) = default;
  friend constexpr bool operator==(partial_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(partial_ordering a, int) { return a.v == -1; }
  friend constexpr bool operator>(partial_ordering a, int) { return a.v == 1; }
  friend constexpr bool operator<=(partial_ordering a, int) { return a.v == -1 || a.v == 0; }
  friend constexpr bool operator>=(partial_ordering a, int) { return a.v == 1 || a.v == 0; }
  friend constexpr bool operator<(int, partial_ordering a) { return a > 0; }
  friend constexpr bool operator>(int, partial_ordering a) { return a < 0; }
  friend constexpr bool operator<=(int, partial_ordering a) { return a >= 0; }
  friend constexpr bool operator>=(int, partial_ordering a) { return a <= 0; }
  friend constexpr partial_ordering operator<=>(int, partial_ordering a) {
    return partial_ordering(a.v == 2 ? 2 : -a.v);
  }
};
inline constexpr partial_ordering partial_ordering::less{-1};
inline constexpr partial_ordering partial_ordering::equivalent{0};
inline constexpr partial_ordering partial_ordering::greater{1};
inline constexpr partial_ordering partial_ordering::unordered{2};
class weak_ordering {
  signed char v;
public:
  constexpr explicit weak_ordering(signed char x) : v(x) {}
  static const weak_ordering less, equivalent, greater;
  constexpr operator partial_ordering() const { return partial_ordering(v); }
  friend constexpr bool operator==(weak_ordering, weak_ordering) = default;
  friend constexpr bool operator==(weak_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(weak_ordering a, int) { return a.v < 0; }
  friend constexpr bool operator>(weak_ordering a, int) { return a.v > 0; }
  friend constexpr bool operator<=(weak_ordering a, int) { return a.v <= 0; }
  friend constexpr bool operator>=(weak_ordering a, int) { return a.v >= 0; }
  friend constexpr bool operator<(int, weak_ordering a) { return a.v > 0; }
  friend constexpr bool operator>(int, weak_ordering a) { return a.v < 0; }
  friend constexpr bool operator<=(int, weak_ordering a) { return a.v >= 0; }
  friend constexpr bool operator>=(int, weak_ordering a) { return a.v <= 0; }
  friend constexpr weak_ordering operator<=>(int, weak_ordering a) { return weak_ordering(-a.v); }
};
inline constexpr weak_ordering weak_ordering::less{-1};
inline constexpr weak_ordering weak_ordering::equivalent{0};
inline constexpr weak_ordering weak_ordering::greater{1};
class strong_ordering {
  signed char v;
public:
  constexpr explicit strong_ordering(signed char x) : v(x) {}
  static const strong_ordering less, equal, equivalent, greater;
  constexpr operator partial_ordering() const { return partial_ordering(v); }
  constexpr operator weak_ordering() const { return weak_ordering(v); }
  friend constexpr bool operator==(strong_ordering, strong_ordering) = default;
  friend constexpr bool operator==(strong_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(strong_ordering a, int) { return a.v < 0; }
  friend constexpr bool operator>(strong_ordering a, int) { return a.v > 0; }
  friend constexpr bool operator<=(strong_ordering a, int) { return a.v <= 0; }
  friend constexpr bool operator>=(strong_ordering a, int) { return a.v >= 0; }
  friend constexpr bool operator<(int, strong_ordering a) { return a.v > 0; }
  friend constexpr bool operator>(int, strong_ordering a) { return a.v < 0; }
  friend constexpr bool operator<=(int, strong_ordering a) { return a.v >= 0; }
  friend constexpr bool operator>=(int, strong_ordering a) { return a.v <= 0; }
  friend constexpr strong_ordering operator<=>(int, strong_ordering a) { return strong_ordering(-a.v); }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
enum class align_val_t : size_t {};
struct nothrow_t { explicit nothrow_t() = default; };
inline constexpr nothrow_t nothrow{};
enum class byte : unsigned char {};
template <class T, class... Args>
constexpr T* construct_at(T* at, Args&&... args) {
  return ::new (static_cast<void*>(at)) T(static_cast<Args&&>(args)...);
}
}
inline void* operator new(std::size_t, void* at) noexcept { return at; }
inline void* operator new[](std::size_t, void* at) noexcept { return at; }
inline void operator delete(void*, void*) noexcept {}
inline void operator delete[](void*, void*) noexcept {}
void* operator new(std::size_t, const std::nothrow_t&) noexcept;
void operator delete(void*, const std::nothrow_t&) noexcept;
void* operator new(std::size_t, std::align_val_t);
void operator delete(void*, std::align_val_t) noexcept;

// --- a program that uses all of it ---
class Range {
  int low, high;
public:
  constexpr Range(int l, int h) : low(l), high(h) {}
  template <std::size_t I> constexpr int get() const { return I == 0 ? low : high; }
};
template <> struct std::tuple_size<Range> { static constexpr size_t value = 2; };
template <> struct std::tuple_size<const Range> { static constexpr size_t value = 2; };
template <std::size_t I> struct std::tuple_element<I, Range> { using type = int; };
template <std::size_t I> struct std::tuple_element<I, const Range> { using type = int; };

struct Version {
  int major, minor;
  auto operator<=>(const Version&) const = default;
};
struct Reading {
  double value;
  auto operator<=>(const Reading&) const = default;
};

constexpr int sum(std::initializer_list<int> xs) { int s = 0; for (int x : xs) s += x; return s; }

constexpr int reset() { int count = 1; std::construct_at(&count, 5); return count; }

struct alignas(32) Wide { int v[8]; };

int main() {
  static_assert(sum({1, 2, 3}) == 6);
  static_assert(Version{1, 2} < Version{1, 3});
  static_assert((Version{2, 0} <=> Version{1, 9}) > 0);
  static_assert(Reading{1.0} < Reading{2.0});
  static_assert(reset() == 5);
  const auto [lo, hi] = Range{1, 5};
  if (lo + hi != 6) return 1;
  int* p = new (std::nothrow) int(1);
  if (p) delete p;
  Wide* w = new Wide{};
  w->v[7] = 3;
  int r = w->v[7];
  delete w;
  if (r != 3) return 2;
  alignas(int) std::byte raw[sizeof(int)];
  int* q = new (raw) int(3);
  if (*q != 3) return 3;
  return 0;
}
