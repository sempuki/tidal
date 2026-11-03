// A lookup table of bit counts, built at compile time.
namespace std { using size_t = decltype(sizeof 0); }

struct BitCounts {
  unsigned char count[256] = {};
};

constexpr BitCounts make_bit_counts() {
  BitCounts t;
  for (std::size_t i = 0; i < 256; ++i) {
    unsigned v = static_cast<unsigned>(i);
    unsigned char n = 0;
    while (v != 0) {
      n += v & 1u;
      v >>= 1;
    }
    t.count[i] = n;
  }
  return t;
}

constexpr BitCounts bit_counts = make_bit_counts();

static_assert(bit_counts.count[0] == 0);
static_assert(bit_counts.count[1] == 1);
static_assert(bit_counts.count[0x0F] == 4);
static_assert(bit_counts.count[0xAA] == 4);
static_assert(bit_counts.count[255] == 8, "all bits set");

constexpr unsigned popcount32(unsigned x) {
  return bit_counts.count[x & 0xFFu] + bit_counts.count[(x >> 8) & 0xFFu] +
         bit_counts.count[(x >> 16) & 0xFFu] + bit_counts.count[(x >> 24) & 0xFFu];
}
static_assert(popcount32(0xFFFFFFFFu) == 32);
static_assert(popcount32(0x80000001u) == 2);

int main() {
  volatile unsigned input = 0xF0F0F0F0u;  // value known only at run time
  unsigned result = popcount32(input);
  return result == 16 ? 0 : 1;
}
