// table.cpp: a consteval function builds lookup tables (CEV-1, CX-P3),
// and a constexpr function reads them (CX-3), checked by static_assert.
namespace std { using size_t = decltype(sizeof 0); }

// A function cannot return an array, so the table is a struct with an
// array member (the CX page example). Every element has a value (CE-21).
template <class T, std::size_t N>
struct Table {
  T v[N] = {};
  constexpr const T& operator[](std::size_t i) const { return v[i]; }
};

// tabulate f [0 .. N-1], in the style of Haskell's listArray.
template <class T, std::size_t N, class F>
consteval Table<T, N> tabulate(F f) {
  Table<T, N> t;
  for (std::size_t i = 0; i < N; ++i) t.v[i] = f(i);
  return t;
}

constexpr auto squares = tabulate<unsigned, 16>([](std::size_t i) { return unsigned(i * i); });

// The CRC-32 table (reflected polynomial 0xEDB88320), 256 entries.
consteval Table<unsigned, 256> crc_table() {
  Table<unsigned, 256> t;
  for (unsigned n = 0; n < 256; ++n) {
    unsigned c = n;
    for (int k = 0; k < 8; ++k) c = (c & 1u) ? (0xEDB88320u ^ (c >> 1)) : (c >> 1);
    t.v[n] = c;
  }
  return t;
}
constexpr auto crc = crc_table();

// Readers: plain constexpr functions, usable in both modes (CX-3).
constexpr unsigned square_of(std::size_t i) { return squares[i]; }

constexpr unsigned crc32(const char* s, std::size_t n) {
  unsigned c = 0xFFFFFFFFu;
  for (std::size_t i = 0; i < n; ++i)
    c = crc[(c ^ static_cast<unsigned char>(s[i])) & 0xFFu] ^ (c >> 8);
  return c ^ 0xFFFFFFFFu;
}

static_assert(square_of(0) == 0);
static_assert(square_of(15) == 225);
static_assert(crc[1] == 0x77073096u);
static_assert(crc[255] == 0x2D02EF8Du);
static_assert(crc32("123456789", 9) == 0xCBF43926u);  // the standard check value
static_assert(crc32("", 0) == 0);

int main(int argc, char** argv) {
  // Run time: the index and the text come from the environment (TR-9).
  std::size_t i = static_cast<std::size_t>(argc) + 2;  // 3 with no arguments
  if (square_of(i) != i * i) return 1;
  const char* digits = "123456789";
  if (argc == 1 && crc32(digits, 9) != 0xCBF43926u) return 2;
  (void)argv;
  return 0;
}
