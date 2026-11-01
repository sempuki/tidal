// W13 task 2: a packed header with bit-fields.
// Rules used: BF-1, BF-3, BF-4, BF-5, BF-6, BF-7, BF-8, BF-9, BF-10, BF-11,
// TYP-1, TYP-2, STG-13, STG-14, STG-26, CAST-1, ENUM (fixed underlying type).

namespace std {
using size_t = decltype(sizeof 0);
}

enum class Kind : unsigned { data = 0, ack = 1, reset = 2, ping = 3 };

// First word: four fields that fill one 32-bit unit (BF-9).
// Second word: a signed field, a kind, and flags. An unnamed bit-field of
// width 0 starts the second word (BF-1).
struct Header {
  unsigned version : 4;
  unsigned length_words : 4;
  unsigned service : 8;
  unsigned total_length : 16;
  unsigned : 0;                 // start a new unit
  int hop_delta : 5;            // BF-5: holds negative values
  Kind kind : 2;                // enumeration bit-field (BF-1)
  bool urgent : 1;
  unsigned reserved : 24;
};

static_assert(sizeof(unsigned) == 4);
static_assert(sizeof(Header) == 8);
static_assert(alignof(Header) == 4);

// A header with 64-bit units. On -m32 the alignment of long long is 4
// (TYP-2). The size stays 8.
struct Wide {
  unsigned long long id : 40;
  unsigned long long seq : 24;
};
static_assert(sizeof(Wide) == 8);
static_assert(alignof(Wide) == (sizeof(void*) == 8 ? 8 : 4));

// BF-6: a field of width 16 promotes to int. A field of width 32 of type
// unsigned promotes to unsigned int.
struct Promo {
  unsigned narrow : 16;
  unsigned full : 32;
};
constexpr int which(int) { return 1; }
constexpr int which(unsigned) { return 2; }
static_assert(which(+Promo{1, 1}.narrow) == 1);
static_assert(which(+Promo{1, 1}.full) == 2);

// Constant evaluation of bit-fields (BF-7, BF-10).
constexpr Header make(unsigned len) {
  Header h{};
  h.version = 4;
  h.length_words = 5;
  h.service = 0;
  h.total_length = len;
  h.hop_delta = -3;
  h.kind = Kind::ack;
  h.urgent = true;
  return h;
}
static_assert(make(1500).total_length == 1500);
static_assert(make(1500).hop_delta == -3);
static_assert(make(1500).kind == Kind::ack);
// BF-10: a value too wide is reduced modulo 2^16 on the matrix targets.
static_assert(make(70000).total_length == 70000 - 65536);

// Portable encoding: the program decides the wire order itself.
void encode(const Header& h, unsigned char out[4]) {
  out[0] = static_cast<unsigned char>((h.version << 4) | h.length_words);
  out[1] = static_cast<unsigned char>(h.service);
  out[2] = static_cast<unsigned char>(h.total_length >> 8);
  out[3] = static_cast<unsigned char>(h.total_length & 0xFFu);
}

Header decode(const unsigned char in[4]) {
  Header h{};
  h.version = in[0] >> 4;
  h.length_words = in[0] & 0x0Fu;
  h.service = in[1];
  h.total_length = (unsigned{in[2]} << 8) | in[3];
  return h;
}

int main() {
  Header h = make(1500);

  unsigned char wire[4];
  encode(h, wire);
  if (wire[0] != 0x45u) return 1;
  if (wire[1] != 0x00u) return 2;
  if (wire[2] != 0x05u || wire[3] != 0xDCu) return 3;

  Header back = decode(wire);
  if (back.version != 4 || back.length_words != 5) return 4;
  if (back.total_length != 1500) return 5;

  // BF-5 at run time: a signed field holds -16 to 15 in 5 bits.
  h.hop_delta = -16;
  if (h.hop_delta != -16) return 6;
  h.hop_delta = h.hop_delta + 31;
  if (h.hop_delta != 15) return 7;

  // BF-10 at run time: an unsigned field wraps modulo 2^4.
  unsigned big = 17;
  h.version = big;
  if (h.version != 1) return 8;

  // BF-4: a const reference binds to a copy of the value.
  const unsigned& copy = h.total_length;
  h.total_length = 9;
  if (copy != 1500) return 9;

  // Look at the bytes of the in-memory layout (STG-13, STG-14).
  // The order of fields inside a unit is not given by the specification.
  // This check uses outside knowledge: the first field takes the low bits.
  Header probe{};
  probe.version = 0xA;
  probe.length_words = 0x5;
  const unsigned char* bytes = reinterpret_cast<const unsigned char*>(&probe);
  if (bytes[0] != 0x5Au) return 10;

  Wide w{};
  w.id = 0xFFFFFFFFFFull;
  w.seq = 0xABCDEFu;
  if (w.id != 0xFFFFFFFFFFull || w.seq != 0xABCDEFu) return 11;

  return 0;
}
