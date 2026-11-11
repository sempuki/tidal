// tidal-rules: OPR-8 OPR-10 PRIM-2
// tidal-expect: run
// tidal-note: Roadmap round 12, programmer sketch bigint_second_try.cpp (reports/clean-room/roadmap-12/programmer.md, section 1.4). The second try builds clean on GCC and Clang, so it is a program test (decision 0061). It differs from the last version in a literal operator template _big over 30-digit literals (Q-0435, Q-0436). The rules are the ones the report names for this program and its reductions.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// BigUint: an unsigned integer of any size in 32-bit limbs on the free
// store, usable in constant evaluation (the storage is freed before the
// evaluation ends). +, *, division by a small number, <=> and ==, decimal
// output, a literal operator template _big, hidden friends, and checks of
// factorials and Fibonacci numbers both in static_assert and at run time.
namespace std {
using size_t = decltype(sizeof 0);
struct strong_ordering {
  signed char v;
  static const strong_ordering less, equal, greater;
  friend constexpr bool operator==(strong_ordering, strong_ordering) = default;
  friend constexpr bool operator==(strong_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(strong_ordering a, int) { return a.v < 0; }
  friend constexpr bool operator>(strong_ordering a, int) { return a.v > 0; }
  friend constexpr bool operator<=(strong_ordering a, int) { return a.v <= 0; }
  friend constexpr bool operator>=(strong_ordering a, int) { return a.v >= 0; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}

using u32 = unsigned;
using u64 = unsigned long long;
static_assert(sizeof(u32) * 8 == 32 && sizeof(u64) * 8 == 64);

template <class T> constexpr void swap(T& a, T& b) {
  T t = static_cast<T&&>(a);
  a = static_cast<T&&>(b);
  b = static_cast<T&&>(t);
}

class BigUint {
  u32* d_ = nullptr;  // little-endian limbs
  std::size_t n_ = 0; // no leading zero limb; zero has n_ == 0
  std::size_t cap_ = 0;

  constexpr void reserve(std::size_t c) {
    if (c <= cap_) return;
    u32* nd = new u32[c];
    for (std::size_t i = 0; i < n_; ++i) nd[i] = d_[i];
    delete[] d_;
    d_ = nd;
    cap_ = c;
  }
  constexpr void trim() {
    while (n_ > 0 && d_[n_ - 1] == 0) --n_;
  }

 public:
  constexpr BigUint() = default;
  constexpr BigUint(u64 v) {
    reserve(2);
    d_[0] = static_cast<u32>(v);
    d_[1] = static_cast<u32>(v >> 32);
    n_ = 2;
    trim();
  }
  constexpr BigUint(const BigUint& o) {
    reserve(o.n_);
    for (std::size_t i = 0; i < o.n_; ++i) d_[i] = o.d_[i];
    n_ = o.n_;
  }
  constexpr BigUint(BigUint&& o) noexcept : d_(o.d_), n_(o.n_), cap_(o.cap_) {
    o.d_ = nullptr;
    o.n_ = o.cap_ = 0;
  }
  constexpr BigUint& operator=(BigUint o) noexcept {
    swap(d_, o.d_);
    swap(n_, o.n_);
    swap(cap_, o.cap_);
    return *this;
  }
  constexpr ~BigUint() { delete[] d_; }

  constexpr std::size_t limbs() const { return n_; }
  constexpr explicit operator bool() const { return n_ != 0; }

  constexpr BigUint& operator+=(const BigUint& o) {
    std::size_t m = n_ > o.n_ ? n_ : o.n_;
    reserve(m + 1);
    for (std::size_t i = n_; i <= m; ++i) d_[i] = 0;
    u64 carry = 0;
    for (std::size_t i = 0; i < m; ++i) {
      u64 s = carry + d_[i] + (i < o.n_ ? o.d_[i] : 0);
      d_[i] = static_cast<u32>(s);
      carry = s >> 32;
    }
    d_[m] = static_cast<u32>(carry);
    n_ = m + 1;
    trim();
    return *this;
  }
  friend constexpr BigUint operator+(BigUint a, const BigUint& b) { return a += b; }

  friend constexpr BigUint operator*(const BigUint& a, const BigUint& b) {
    BigUint r;
    if (!a || !b) return r;
    r.reserve(a.n_ + b.n_);
    for (std::size_t i = 0; i < a.n_ + b.n_; ++i) r.d_[i] = 0;
    for (std::size_t i = 0; i < a.n_; ++i) {
      u64 carry = 0;
      for (std::size_t j = 0; j < b.n_; ++j) {
        u64 t = static_cast<u64>(a.d_[i]) * b.d_[j] + r.d_[i + j] + carry;
        r.d_[i + j] = static_cast<u32>(t);
        carry = t >> 32;
      }
      r.d_[i + b.n_] = static_cast<u32>(carry);
    }
    r.n_ = a.n_ + b.n_;
    r.trim();
    return r;
  }
  constexpr BigUint& operator*=(const BigUint& o) { return *this = *this * o; }

  // Divides in place and returns the remainder.
  constexpr u32 divmod(u32 by) {
    u64 rem = 0;
    for (std::size_t i = n_; i-- > 0;) {
      u64 cur = (rem << 32) | d_[i];
      d_[i] = static_cast<u32>(cur / by);
      rem = cur % by;
    }
    trim();
    return static_cast<u32>(rem);
  }

  friend constexpr std::strong_ordering operator<=>(const BigUint& a, const BigUint& b) {
    if (a.n_ != b.n_) return a.n_ < b.n_ ? std::strong_ordering::less : std::strong_ordering::greater;
    for (std::size_t i = a.n_; i-- > 0;)
      if (a.d_[i] != b.d_[i]) return a.d_[i] < b.d_[i] ? std::strong_ordering::less : std::strong_ordering::greater;
    return std::strong_ordering::equal;
  }
  friend constexpr bool operator==(const BigUint& a, const BigUint& b) { return (a <=> b) == 0; }

  // Writes the decimal digits and a terminating zero; returns the count.
  constexpr std::size_t to_chars(char* out, std::size_t room) const {
    if (!*this) { out[0] = '0'; out[1] = 0; return 1; }
    BigUint t = *this;
    std::size_t k = 0;
    while (t && k + 1 < room) out[k++] = static_cast<char>('0' + t.divmod(10));
    for (std::size_t i = 0; i < k / 2; ++i) swap(out[i], out[k - 1 - i]);
    out[k] = 0;
    return k;
  }
};

template <char... Cs>
constexpr BigUint operator""_big() {
  BigUint r;
  ((r = r * 10u + static_cast<u64>(Cs - '0')), ...);
  return r;
}

constexpr bool same(const char* a, const char* b) {
  while (*a && *a == *b) { ++a; ++b; }
  return *a == *b;
}

constexpr BigUint factorial(u32 n) {
  BigUint r = 1u;
  for (u32 i = 2; i <= n; ++i) r *= i;
  return r;
}

constexpr BigUint fib(u32 n) {
  BigUint a = 0u, b = 1u;
  for (u32 i = 0; i < n; ++i) {
    BigUint c = a + b;
    a = static_cast<BigUint&&>(b);
    b = static_cast<BigUint&&>(c);
  }
  return a;
}

constexpr bool decimal_is(const BigUint& v, const char* expect) {
  char buf[128] = {};
  v.to_chars(buf, sizeof buf);
  return same(buf, expect);
}

static_assert(decimal_is(factorial(25), "15511210043330985984000000"));
static_assert(decimal_is(fib(100), "354224848179261915075"));
static_assert(factorial(20) == 2432902008176640000ull);
// Second try: BigUint has no operator/, so divide a copy with divmod.
static_assert(factorial(21) > factorial(20) && [] {
  BigUint t = factorial(21);
  return t.divmod(21) == 0 && t == factorial(20);
}());
static_assert(123456789012345678901234567890_big + 1u == 123456789012345678901234567891_big);

int main() {
  int fails = 0;
  BigUint f = factorial(30);
  if (!decimal_is(f, "265252859812191058636308480000000")) ++fails;
  BigUint g = f;
  u32 r = g.divmod(1000000007u);
  // 30! mod 1e9+7, computed by hand from the product of 1..30.
  u64 m = 1;
  for (u64 i = 1; i <= 30; ++i) m = m * i % 1000000007u;
  if (r != m) ++fails;
  BigUint h = fib(200);
  if (!decimal_is(h, "280571172992510140037611932413038677189525")) ++fails;
  if (!(h > f) || h == f) ++fails;
  BigUint z;
  if (z || !decimal_is(z, "0")) ++fails;
  BigUint big = 99999999999999999999999999_big * 99999999999999999999999999_big;
  if (!decimal_is(big, "9999999999999999999999999800000000000000000000000001")) ++fails;
  return fails;
}
