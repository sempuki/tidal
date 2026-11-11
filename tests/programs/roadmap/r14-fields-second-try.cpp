// tidal-rules: LIST-4 LIST-12 CPT-26 IF-13 LAM-5 LAM-18
// tidal-expect: run
// tidal-note: Roadmap round 14, programmer sketch fields_second_try.cpp (reports/clean-room/roadmap-14/programmer.md, section 1.4). The second try builds clean on GCC and Clang, so it is a program test (decision 0061). It differs from the last version in an encoder and decoder that capture the buffer by reference in place of through a pointer. The rules are the ones the report names for this program and its reductions.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Field-wise operations on plain aggregates with no macros: the field
// count found by trying braced initialization with a type that converts to
// anything, the fields reached by structured bindings, and on top of that
// equality, a hash, and a byte encoder and decoder that round-trip.
// No standard header.

using size_t = decltype(sizeof 0);
using u8 = unsigned char;
using u32 = unsigned;
using u64 = unsigned long long;

namespace fields {

template <class T> T&& declval() noexcept;

template <class A, class B> inline constexpr bool same = false;
template <class A> inline constexpr bool same<A, A> = true;

template <class T> struct remove_cvref { using type = T; };
template <class T> struct remove_cvref<const T> { using type = T; };
template <class T> struct remove_cvref<T&> { using type = typename remove_cvref<T>::type; };
template <class T> struct remove_cvref<T&&> { using type = typename remove_cvref<T>::type; };
template <class T> using remove_cvref_t = typename remove_cvref<T>::type;

// Converts to any type. Used only in unevaluated braced initialization.
struct any {
  template <class T>
  operator T() const;
};

template <class T, class... A>
concept brace_init = requires { T{declval<A>()...}; };

template <class T, class... A>
constexpr size_t count_from() {
  if constexpr (brace_init<T, A..., any>)
    return count_from<T, A..., any>();
  else
    return sizeof...(A);
}

template <class T>
inline constexpr size_t count = count_from<T>();

// Calls f with a reference to each field, in order.
template <class T, class F>
constexpr void for_each(T&& t, F&& f) {
  constexpr size_t n = count<remove_cvref_t<T>>;
  static_assert(n <= 4, "for_each handles at most four fields");
  if constexpr (n == 1) {
    auto&& [a] = t;
    f(a);
  } else if constexpr (n == 2) {
    auto&& [a, b] = t;
    f(a); f(b);
  } else if constexpr (n == 3) {
    auto&& [a, b, c] = t;
    f(a); f(b); f(c);
  } else if constexpr (n == 4) {
    auto&& [a, b, c, d] = t;
    f(a); f(b); f(c); f(d);
  }
}

// Calls f with two references, one field of each object, in order.
template <class T, class F>
constexpr void for_each2(const T& x, const T& y, F&& f) {
  constexpr size_t n = count<T>;
  if constexpr (n == 1) {
    auto& [a] = x; auto& [p] = y;
    f(a, p);
  } else if constexpr (n == 2) {
    auto& [a, b] = x; auto& [p, q] = y;
    f(a, p); f(b, q);
  } else if constexpr (n == 3) {
    auto& [a, b, c] = x; auto& [p, q, r] = y;
    f(a, p); f(b, q); f(c, r);
  } else if constexpr (n == 4) {
    auto& [a, b, c, d] = x; auto& [p, q, r, s] = y;
    f(a, p); f(b, q); f(c, r); f(d, s);
  }
}

template <class T>
concept integral = same<T, bool> || same<T, char> || same<T, short> || same<T, int> ||
                   same<T, long> || same<T, long long> || same<T, unsigned char> ||
                   same<T, unsigned short> || same<T, unsigned> || same<T, unsigned long> ||
                   same<T, unsigned long long>;

template <class T>
concept record = !integral<T> && requires { count<T>; } && (count<T> > 0);

template <class T>
constexpr bool equal(const T& x, const T& y) {
  if constexpr (integral<T>) {
    return x == y;
  } else {
    bool eq = true;
    for_each2(x, y, [&](const auto& a, const auto& b) { eq = eq && equal(a, b); });
    return eq;
  }
}

template <class T>
constexpr u64 hash(const T& x, u64 h = 1469598103934665603ull) {
  if constexpr (integral<T>) {
    u64 v = static_cast<u64>(x);
    for (size_t i = 0; i < sizeof(T); ++i) {
      h ^= (v >> (8 * i)) & 0xff;
      h *= 1099511628211ull;
    }
    return h;
  } else {
    for_each(x, [&](const auto& f) { h = hash(f, h); });
    return h;
  }
}

// A byte buffer with a write and a read cursor.
template <size_t N>
struct Buffer {
  u8 bytes[N]{};
  size_t wpos = 0;
  size_t rpos = 0;
  bool overflow = false;

  constexpr void put(u8 b) {
    if (wpos == N) { overflow = true; return; }
    bytes[wpos++] = b;
  }
  constexpr u8 get() { return rpos < wpos ? bytes[rpos++] : 0; }
};

template <class T, size_t N>
constexpr void encode(const T& x, Buffer<N>& out) {
  if constexpr (integral<T>) {
    u64 v = static_cast<u64>(x);
    for (size_t i = 0; i < sizeof(T); ++i) out.put(static_cast<u8>(v >> (8 * i)));
  } else {
    for_each(x, [&](const auto& f) { encode(f, out); });
  }
}

template <class T, size_t N>
constexpr void decode(T& x, Buffer<N>& in) {
  if constexpr (integral<T>) {
    u64 v = 0;
    for (size_t i = 0; i < sizeof(T); ++i) v |= static_cast<u64>(in.get()) << (8 * i);
    x = static_cast<T>(v);
  } else {
    for_each(x, [&](auto& f) { decode(f, in); });
  }
}

}  // namespace fields

struct Color { u8 r, g, b; };
struct Pixel { short x, y; Color c; };
struct Sample { int id; bool valid; long long value; Pixel at; };
struct Empty {};

// Workaround: the counts by hand, for the aggregates with a class member.
template <> inline constexpr size_t fields::count<Pixel> = 3;
template <> inline constexpr size_t fields::count<Sample> = 4;

static_assert(fields::count<Color> == 3);
static_assert(fields::count<Pixel> == 3);
static_assert(fields::count<Sample> == 4);
static_assert(fields::count<Empty> == 0);
static_assert(fields::record<Pixel>);
static_assert(!fields::record<int>);
static_assert(!fields::record<Empty>);

constexpr Sample make(int id) {
  return Sample{id, id % 2 == 0, -1234567890123ll * id, Pixel{short(id), short(-id), Color{1, 2, u8(id)}}};
}

constexpr bool round_trip() {
  fields::Buffer<64> buf;
  Sample s = make(4);
  fields::encode(s, buf);
  if (buf.overflow) return false;
  if (buf.wpos != sizeof(int) + 1 + sizeof(long long) + 2 * sizeof(short) + 3) return false;
  Sample t{};
  fields::decode(t, buf);
  return fields::equal(s, t) && t.at.c.b == 4 && t.value == -4938271560492ll;
}

constexpr bool hashes() {
  return fields::hash(make(1)) == fields::hash(make(1)) &&
         fields::hash(make(1)) != fields::hash(make(2)) &&
         fields::hash(Color{1, 2, 3}) != fields::hash(Color{3, 2, 1});
}

constexpr bool overflows() {
  fields::Buffer<8> small;
  fields::encode(make(3), small);
  return small.overflow && small.wpos == 8;
}

static_assert(round_trip());
static_assert(hashes());
static_assert(overflows());
static_assert(fields::equal(Color{1, 2, 3}, Color{1, 2, 3}));
static_assert(!fields::equal(Pixel{1, 2, {3, 4, 5}}, Pixel{1, 2, {3, 4, 6}}));

int main() {
  int r = 0;
  if (!round_trip()) r |= 1;
  if (!hashes()) r |= 2;
  if (!overflows()) r |= 4;
  fields::Buffer<32> b;
  Pixel p{7, 8, {9, 10, 11}}, q{};
  fields::encode(p, b);
  fields::decode(q, b);
  if (!fields::equal(p, q)) r |= 8;
  return r;
}
