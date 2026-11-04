// tidal-rules: LSUP-4 TYP-1 UN-12 SHF-4 SHF-7 ASN-5 BIT-5 BIT-4 BIT-1 ADD-5
// tidal-expect: run
// tidal-note: Write suite W1, program bitset.cpp (reports/clean-room/W1/report.md). The report has no construct-to-rule notes, so the rules are the ones the program's comments name.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// A bit set over an array of unsigned words.

namespace std {
using size_t = decltype(sizeof 0);  // LSUP-4, the shape from its page
}

using Word = unsigned long long;     // TYP-1: 8 bytes on x86-64 and -m32
constexpr std::size_t word_bits = sizeof(Word) * 8;  // UN-12

template <std::size_t Bits>
class BitSet {
public:
  static constexpr std::size_t word_count = (Bits + word_bits - 1) / word_bits;

  constexpr BitSet() : words_{} {}   // value-initialized: every word is 0

  constexpr std::size_t size() const { return Bits; }

  // SHF-4: the result type is the promoted left operand, so the 1 must
  // already be a Word. A plain 1 is an int, and 1 << 40 is undefined
  // (SHF-7).
  static constexpr Word mask(std::size_t i) { return Word{1} << (i % word_bits); }

  // Returns false for an index out of range, like Rust's get() -> None.
  constexpr bool set(std::size_t i) {
    if (i >= Bits) return false;
    words_[i / word_bits] |= mask(i);   // ASN-5, BIT-5
    return true;
  }

  constexpr bool reset(std::size_t i) {
    if (i >= Bits) return false;
    words_[i / word_bits] &= ~mask(i);  // BIT-4: ~ on Word keeps Word
    return true;
  }

  constexpr bool flip(std::size_t i) {
    if (i >= Bits) return false;
    words_[i / word_bits] ^= mask(i);
    return true;
  }

  // BIT-1: == binds more tightly than &, so the parentheses matter.
  constexpr bool test(std::size_t i) const {
    if (i >= Bits) return false;
    return (words_[i / word_bits] & mask(i)) != 0;
  }

  // Kernighan's loop: w & (w - 1) clears the lowest set bit. w - 1 on
  // 0 would wrap (ADD-5), but the loop stops before that.
  constexpr std::size_t count() const {
    std::size_t n = 0;
    for (std::size_t k = 0; k < word_count; ++k) {
      Word w = words_[k];
      while (w != 0) {
        w &= w - 1;
        ++n;
      }
    }
    return n;
  }

  // The index of the first set bit at or after `from`, or Bits if none.
  constexpr std::size_t next(std::size_t from) const {
    for (std::size_t i = from; i < Bits; ++i) {
      if (test(i)) return i;
    }
    return Bits;
  }

  // Complement. The bits past Bits in the last word stay 0, so count()
  // and operator== are not confused by them.
  constexpr BitSet operator~() const {
    BitSet r;
    for (std::size_t k = 0; k < word_count; ++k) r.words_[k] = ~words_[k];
    r.clear_tail();
    return r;
  }

  constexpr BitSet operator&(const BitSet& o) const {
    BitSet r;
    for (std::size_t k = 0; k < word_count; ++k) r.words_[k] = words_[k] & o.words_[k];
    return r;
  }

  constexpr BitSet operator|(const BitSet& o) const {
    BitSet r;
    for (std::size_t k = 0; k < word_count; ++k) r.words_[k] = words_[k] | o.words_[k];
    return r;
  }

  constexpr BitSet operator^(const BitSet& o) const {
    BitSet r;
    for (std::size_t k = 0; k < word_count; ++k) r.words_[k] = words_[k] ^ o.words_[k];
    return r;
  }

  constexpr bool operator==(const BitSet& o) const {
    for (std::size_t k = 0; k < word_count; ++k) {
      if (words_[k] != o.words_[k]) return false;
    }
    return true;
  }

  constexpr Word word(std::size_t k) const { return words_[k]; }

private:
  constexpr void clear_tail() {
    constexpr std::size_t used = Bits % word_bits;
    if (used != 0) {
      // used is 1..63, so the shift count is in range (SHF-7).
      words_[word_count - 1] &= (Word{1} << used) - 1;
    }
  }

  Word words_[word_count];
};

constexpr BitSet<100> primes_below_100() {
  BitSet<100> s;
  for (std::size_t i = 2; i < 100; ++i) s.set(i);
  for (std::size_t i = 2; i * i < 100; ++i) {
    if (s.test(i)) {
      for (std::size_t j = i * i; j < 100; j += i) s.reset(j);
    }
  }
  return s;
}

constexpr BitSet<100> primes = primes_below_100();
static_assert(primes.count() == 25);
static_assert(primes.test(97) && !primes.test(91));
static_assert(primes.next(90) == 97);
static_assert(primes.next(98) == 100);
static_assert(!primes.test(100));          // out of range gives false
static_assert((~primes).count() == 75);    // tail bits stay clear
static_assert((primes | ~primes).count() == 100);
static_assert((primes & ~primes).count() == 0);
static_assert((primes ^ primes) == BitSet<100>());
static_assert(BitSet<64>::word_count == 1 && BitSet<65>::word_count == 2);
static_assert(BitSet<64>::mask(63) == 0x8000'0000'0000'0000u);

constexpr bool high_bits() {
  BitSet<128> s;
  s.set(63);
  s.set(64);
  s.set(127);
  return s.word(0) == 0x8000'0000'0000'0000u && s.word(1) == 0x8000'0000'0000'0001u &&
         s.count() == 3;
}
static_assert(high_bits());

int main(int argc, char**) {
  BitSet<200> s;
  std::size_t step = static_cast<std::size_t>(argc) * 3;  // 3
  for (std::size_t i = 0; i < s.size(); i += step) s.set(i);
  if (s.count() != 67) return 1;          // 0, 3, ..., 198
  if (!s.test(198) || s.test(199)) return 2;
  if (s.set(200)) return 3;               // refused
  s.flip(199);
  if (s.count() != 68) return 4;
  BitSet<200> t = ~s;
  if (t.count() != 132) return 5;
  if ((s | t).count() != 200) return 6;
  std::size_t visited = 0;
  for (std::size_t i = s.next(0); i < s.size(); i = s.next(i + 1)) ++visited;
  if (visited != 68) return 7;
  return 0;
}
