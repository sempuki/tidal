// Bits<N>: a fixed-size set of bits in an array of 64-bit words, with a
// proxy reference for b[i] = true, shifts across words, the bit
// operators as hidden friends, count, a constructor from a string of
// '0' and '1', and a range over the indices of the set bits with an
// end type of its own. Constant tests and a sieve of primes.
namespace std {
using size_t = decltype(sizeof 0);
}
using u64 = unsigned long long;

template <std::size_t N>
class Bits {
  static constexpr std::size_t W = 64;
  static constexpr std::size_t words = N == 0 ? 1 : (N + W - 1) / W;
  u64 w_[words]{};

  constexpr void trim() {
    if constexpr (N % W != 0) w_[words - 1] &= (u64{1} << (N % W)) - 1;
  }

public:
  class reference {
    friend class Bits;
    u64* word_;
    u64 mask_;
    constexpr reference(u64* w, u64 m) : word_(w), mask_(m) {}

  public:
    constexpr reference& operator=(bool b) {
      if (b) *word_ |= mask_;
      else *word_ &= ~mask_;
      return *this;
    }
    constexpr reference& operator=(const reference& r) { return *this = static_cast<bool>(r); }
    constexpr operator bool() const { return (*word_ & mask_) != 0; }
    constexpr bool operator~() const { return (*word_ & mask_) == 0; }
    constexpr reference& flip() { *word_ ^= mask_; return *this; }
    friend constexpr void swap(reference a, reference b) {
      bool t = a;
      a = static_cast<bool>(b);
      b = t;
    }
  };

  constexpr Bits() = default;
  constexpr explicit Bits(u64 v) {
    w_[0] = v;
    trim();
  }
  template <std::size_t M>
  constexpr explicit Bits(const char (&s)[M]) {
    // s[0] is the highest bit, as in a binary literal.
    std::size_t len = M - 1;
    for (std::size_t i = 0; i < len && i < N; ++i)
      if (s[len - 1 - i] == '1') set(i);
  }

  static constexpr std::size_t size() { return N; }

  constexpr bool operator[](std::size_t i) const { return (w_[i / W] >> (i % W)) & 1; }
  constexpr reference operator[](std::size_t i) { return reference(&w_[i / W], u64{1} << (i % W)); }

  constexpr Bits& set(std::size_t i, bool v = true) {
    (*this)[i] = v;
    return *this;
  }
  constexpr Bits& reset(std::size_t i) { return set(i, false); }
  constexpr Bits& flip() {
    for (auto& w : w_) w = ~w;
    trim();
    return *this;
  }

  constexpr std::size_t count() const {
    std::size_t c = 0;
    for (u64 w : w_)
      for (; w; w &= w - 1) ++c;
    return c;
  }
  constexpr bool any() const { return count() != 0; }
  constexpr bool none() const { return !any(); }
  constexpr bool all() const { return count() == N; }

  constexpr Bits& operator<<=(std::size_t s) {
    if (s >= N) { *this = Bits(); return *this; }
    const std::size_t ws = s / W, bs = s % W;
    for (std::size_t i = words; i-- > 0;) {
      u64 v = 0;
      if (i >= ws) {
        v = w_[i - ws] << bs;
        if (bs && i > ws) v |= w_[i - ws - 1] >> (W - bs);
      }
      w_[i] = v;
    }
    trim();
    return *this;
  }
  constexpr Bits& operator>>=(std::size_t s) {
    if (s >= N) { *this = Bits(); return *this; }
    const std::size_t ws = s / W, bs = s % W;
    for (std::size_t i = 0; i < words; ++i) {
      u64 v = 0;
      if (i + ws < words) {
        v = w_[i + ws] >> bs;
        if (bs && i + ws + 1 < words) v |= w_[i + ws + 1] << (W - bs);
      }
      w_[i] = v;
    }
    return *this;
  }
  constexpr Bits& operator&=(const Bits& o) { for (std::size_t i = 0; i < words; ++i) w_[i] &= o.w_[i]; return *this; }
  constexpr Bits& operator|=(const Bits& o) { for (std::size_t i = 0; i < words; ++i) w_[i] |= o.w_[i]; return *this; }
  constexpr Bits& operator^=(const Bits& o) { for (std::size_t i = 0; i < words; ++i) w_[i] ^= o.w_[i]; return *this; }

  friend constexpr Bits operator<<(Bits b, std::size_t s) { return b <<= s; }
  friend constexpr Bits operator>>(Bits b, std::size_t s) { return b >>= s; }
  friend constexpr Bits operator&(Bits a, const Bits& b) { return a &= b; }
  friend constexpr Bits operator|(Bits a, const Bits& b) { return a |= b; }
  friend constexpr Bits operator^(Bits a, const Bits& b) { return a ^= b; }
  friend constexpr Bits operator~(Bits a) { return a.flip(); }
  friend constexpr bool operator==(const Bits&, const Bits&) = default;

  constexpr u64 to_u64() const { return w_[0]; }

  // The indices of the set bits, lowest first.
  struct End {};
  class OnesIterator {
    const Bits* b_;
    std::size_t i_;
    constexpr void skip() { while (i_ < N && !(*b_)[i_]) ++i_; }

  public:
    constexpr OnesIterator(const Bits* b, std::size_t i) : b_(b), i_(i) { skip(); }
    constexpr std::size_t operator*() const { return i_; }
    constexpr OnesIterator& operator++() { ++i_; skip(); return *this; }
    constexpr bool operator==(End) const { return i_ >= N; }
  };
  struct Ones {
    const Bits* b;
    constexpr OnesIterator begin() const { return {b, 0}; }
    constexpr End end() const { return {}; }
  };
  constexpr Ones ones() const { return {this}; }
};

// Constant tests.
static_assert(Bits<8>("1011").to_u64() == 11);
static_assert(Bits<8>(0xFFF).to_u64() == 0xFF);
static_assert(Bits<130>().set(129).count() == 1);
static_assert((Bits<130>().set(0) << 129)[129]);
static_assert(((Bits<130>().set(0) << 129) >> 129)[0]);
static_assert((Bits<130>().set(63) << 1)[64]);
static_assert((Bits<130>().set(64) >> 1)[63]);
static_assert(Bits<70>().flip().all() && Bits<70>().flip().count() == 70);
static_assert((~Bits<5>(0b10101)).to_u64() == 0b01010);
static_assert((Bits<16>(0xF0F0) & Bits<16>(0xFF00)) == Bits<16>(0xF000));
static_assert((Bits<16>(0xF0F0) ^ Bits<16>(0xFF00)).to_u64() == 0x0FF0);
static_assert(Bits<0>().none() && Bits<0>().all());

constexpr int sum_of_ones(const Bits<200>& b) {
  int s = 0;
  for (std::size_t i : b.ones()) s += static_cast<int>(i);
  return s;
}
static_assert(sum_of_ones(Bits<200>().set(3).set(64).set(199)) == 266);

constexpr bool proxy_test() {
  Bits<10> b;
  b[2] = true;
  b[5] = static_cast<bool>(b[2]);
  b[2].flip();
  swap(b[5], b[7]);
  return !b[2] && !b[5] && b[7] && ~b[0] && b.count() == 1;
}
static_assert(proxy_test());

template <std::size_t N> constexpr Bits<N> primes() {
  Bits<N> p;
  p.flip();
  p.reset(0);
  p.reset(1);
  for (std::size_t i = 2; i * i < N; ++i)
    if (p[i])
      for (std::size_t j = i * i; j < N; j += i) p[j] = false;
  return p;
}
static_assert(primes<100>().count() == 25);

int main() {
  constexpr auto p = primes<1000>();
  if (p.count() != 168) return 1;
  std::size_t last = 0;
  int n = 0;
  for (std::size_t i : p.ones()) {
    last = i;
    ++n;
  }
  if (n != 168 || last != 997) return 2;
  Bits<1000> q = p;
  q <<= 1;
  if (!q[998] || q[997]) return 3;
  q >>= 1;
  if (q != p) return 4;
  Bits<1000> evens;
  for (std::size_t i = 0; i < 1000; i += 2) evens[i] = true;
  if ((p & evens).count() != 1) return 5;
  if ((p | evens).count() != 500 + 167) return 6;
  return 0;
}
