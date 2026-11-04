// tidal-rules: LSUP-4 TYP-1 CONV-1 CONV-19 CONV-2 SHF-5 PRIM-1 UN-3 BIT-4 LIST-6 TYP-4 TYP-3 TR-5
// tidal-expect: run
// tidal-note: Write suite W1, program checksum.cpp (reports/clean-room/W1/report.md). The report has no construct-to-rule notes, so the rules are the ones the program's comments name.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Checksums over a byte array: Fletcher-16, Adler-32, and CRC-32.

namespace std {
using size_t = decltype(sizeof 0);  // LSUP-4
}

using u8 = unsigned char;   // TYP-1: size 1
using u16 = unsigned short; // TYP-1: 2 bytes
using u32 = unsigned int;   // TYP-1: 4 bytes on both matrix targets

static_assert(sizeof(u16) == 2 && sizeof(u32) == 4);  // TYP-P3

// Fletcher-16. CONV-1: data[i] promotes to int, and CONV-19 step 4
// makes each sum u32. The % keeps the sums below 255. The final value
// converts back to u16 (CONV-2).
constexpr u16 fletcher16(const u8* data, std::size_t len) {
  u32 a = 0, b = 0;
  for (std::size_t i = 0; i < len; ++i) {
    a = (a + data[i]) % 255;
    b = (b + a) % 255;
  }
  return static_cast<u16>((b << 8) | a);
}

// Adler-32. All arithmetic stays in u32: u8 promotes to int (CONV-1),
// then int and unsigned int give unsigned int (CONV-19 step 4).
constexpr u32 adler32(const u8* data, std::size_t len) {
  constexpr u32 mod = 65521;
  u32 a = 1, b = 0;
  for (std::size_t i = 0; i < len; ++i) {
    a = (a + data[i]) % mod;
    b = (b + a) % mod;
  }
  return (b << 16) | a;  // SHF-5: unsigned, so the high bits of b drop
}

// CRC-32 (reflected, polynomial 0xEDB88320), bit by bit.
// PRIM-1: 0xEDB88320 does not fit in int, so the hex literal is
// unsigned int.
constexpr u32 crc32(const u8* data, std::size_t len) {
  u32 crc = 0xFFFF'FFFFu;
  for (std::size_t i = 0; i < len; ++i) {
    crc ^= data[i];
    for (int k = 0; k < 8; ++k) {
      // -(crc & 1) is 0 or all ones (UN-3: unary minus on unsigned
      // is 2^32 minus the value).
      u32 mask = -(crc & 1u);
      crc = (crc >> 1) ^ (0xEDB88320u & mask);
    }
  }
  return ~crc;  // BIT-4: u32 is not promoted, so ~ stays u32
}

// A table-driven CRC-32 built at compile time, checked against the
// bitwise one.
struct CrcTable {
  u32 entry[256];
};

constexpr CrcTable make_crc_table() {
  CrcTable t{};
  for (u32 n = 0; n < 256; ++n) {
    u32 c = n;
    for (int k = 0; k < 8; ++k) c = (c & 1u) ? (0xEDB88320u ^ (c >> 1)) : (c >> 1);
    t.entry[n] = c;
  }
  return t;
}

constexpr CrcTable crc_table = make_crc_table();

constexpr u32 crc32_table(const u8* data, std::size_t len) {
  u32 crc = ~0u;
  for (std::size_t i = 0; i < len; ++i) {
    crc = crc_table.entry[(crc ^ data[i]) & 0xFFu] ^ (crc >> 8);
  }
  return ~crc;
}

// LIST-6: a plain string literal initializes an unsigned char array.
// The bound counts the terminating null, so the length is sizeof - 1.
constexpr u8 digits[] = "123456789";
constexpr u8 wiki[] = "Wikipedia";
constexpr u8 abcde[] = "abcde";
constexpr std::size_t digits_len = sizeof digits - 1;

static_assert(digits_len == 9);
static_assert(crc32(digits, digits_len) == 0xCBF43926u);
static_assert(crc32_table(digits, digits_len) == 0xCBF43926u);
static_assert(crc32(digits, 0) == 0);
static_assert(adler32(wiki, sizeof wiki - 1) == 0x11E60398u);
static_assert(adler32(digits, 0) == 1);
static_assert(fletcher16(abcde, 5) == 0xC8F0);

// Bytes above 127. u8 holds 0..255 (TYP-4); a char would be negative
// here (TYP-3), which is why the checksums take u8.
constexpr u8 high[] = {0xFF, 0x80, 0x00, 0x7F};
static_assert(crc32(high, 4) == crc32_table(high, 4));
static_assert(fletcher16(high, 4) == 0x0100);  // a = 0, b = 1

int main(int argc, char**) {
  // A run-time buffer: 1000 bytes with the pattern i * 7 + argc.
  u8 buf[1000];
  for (std::size_t i = 0; i < sizeof buf; ++i) {
    buf[i] = static_cast<u8>(i * 7 + static_cast<std::size_t>(argc));  // CONV-2: low 8 bits
  }
  if (crc32(buf, sizeof buf) != crc32_table(buf, sizeof buf)) return 1;

  // A checksum changes when one bit flips.
  u32 before = adler32(buf, sizeof buf);
  u32 crc_before = crc32(buf, sizeof buf);
  buf[500] ^= 0x10;
  if (adler32(buf, sizeof buf) == before) return 2;
  if (crc32(buf, sizeof buf) == crc_before) return 3;
  buf[500] ^= 0x10;
  if (adler32(buf, sizeof buf) != before) return 4;

  // The same known answer at run time as at compile time (TR-5).
  if (crc32(digits, digits_len) != 0xCBF43926u) return 5;
  return 0;
}
