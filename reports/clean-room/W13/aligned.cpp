// W13 task 4: a buffer aligned for a wider type, with objects placed in it.
// Rules used: ALN-1, ALN-2, ALN-4, ALN-5, TYP-1, TYP-2, STG-4, STG-5,
// STG-6, STG-16, STG-17, NEW-26, NEW-28, NEW-33, LSUP-4, LSUP-7, CAST-6,
// CAST-16, ADD-6, DTOR-6, OBJ-1.

namespace std {
using size_t = decltype(sizeof 0);
}

inline void* operator new(std::size_t, void* at) noexcept { return at; }
inline void* operator new[](std::size_t, void* at) noexcept { return at; }
inline void operator delete(void*, void*) noexcept {}
inline void operator delete[](void*, void*) noexcept {}

// CAST-16: on the matrix targets the integer is the address. TYP-1: the
// size type has the size of a pointer on both targets.
static_assert(sizeof(std::size_t) == sizeof(void*));
std::size_t address_of(const void* p) {
  return reinterpret_cast<std::size_t>(p);
}
bool aligned_to(const void* p, std::size_t a) { return address_of(p) % a == 0; }

int live = 0;   // count of live Sample objects

struct Sample {
  long long time;
  double value;
  Sample(long long t, double v) : time(t), value(v) { ++live; }
  ~Sample() { --live; }
};

// An over-aligned type. 32 is more than the largest fundamental alignment
// on the matrix targets (ALN-5), so this is an extended alignment.
struct alignas(32) Block {
  float lanes[4];
};
static_assert(alignof(Block) == 32);
static_assert(sizeof(Block) == 32);        // ALN-1: size is a multiple

// TYP-2: long long and double have alignment 8 on x86-64 and 4 on -m32.
static_assert(alignof(Sample) == (sizeof(void*) == 8 ? 8 : 4));

// The buffer. ALN-1: alignas on a variable. It is aligned for Block,
// which is the widest type placed in it, and so for Sample and double too.
constexpr std::size_t buffer_size = 128;
alignas(Block) unsigned char buffer[buffer_size];

// Round an offset up to a multiple of a power of two.
constexpr std::size_t align_up(std::size_t offset, std::size_t alignment) {
  return (offset + alignment - 1) & ~(alignment - 1);
}
static_assert(align_up(1, 8) == 8);
static_assert(align_up(16, 8) == 16);

int main() {
  if (!aligned_to(buffer, 32)) return 1;          // ALN-4, static storage

  // 1. An array of double at the start (STG-5, NEW-33: no extra storage).
  double* values = new (buffer) double[4]{1.0, 2.0, 3.0, 4.0};
  if (static_cast<void*>(values) != static_cast<void*>(buffer)) return 2;
  if (values[0] + values[3] != 5.0) return 3;

  // 2. Reuse the bytes for objects of other types (STG-6). Carve them one
  // after another, each at an offset aligned for its own type.
  std::size_t offset = 0;
  unsigned char* tag = new (buffer + offset) unsigned char(0x7F);
  offset += sizeof(unsigned char);

  offset = align_up(offset, alignof(Sample));
  Sample* first = new (buffer + offset) Sample(100, 0.5);
  offset += sizeof(Sample);

  offset = align_up(offset, alignof(Block));
  if (offset + sizeof(Block) > buffer_size) return 4;
  Block* block = new (buffer + offset) Block{{1.0f, 2.0f, 3.0f, 4.0f}};
  offset += sizeof(Block);

  if (!aligned_to(first, alignof(Sample))) return 5;
  if (!aligned_to(block, 32)) return 6;
  if (*tag != 0x7F) return 7;
  if (first->time != 100 || first->value != 0.5) return 8;
  if (block->lanes[2] != 3.0f) return 9;
  if (live != 1) return 10;

  // 3. End the Sample. DTOR-6: an explicit destructor call. The bytes
  // stay and can hold a new Sample.
  first->~Sample();
  if (live != 0) return 11;
  Sample* second = new (static_cast<void*>(first)) Sample(200, 1.5);
  if (second->time != 200 || live != 1) return 12;
  second->~Sample();

  // 4. An over-aligned object in dynamic storage (ALN-4, NEW-28).
  Block* dynamic = new Block{{5.0f, 6.0f, 7.0f, 8.0f}};
  if (!aligned_to(dynamic, 32)) return 13;
  if (dynamic->lanes[3] != 8.0f) return 14;
  delete dynamic;

  // 5. An over-aligned local (ALN-4, automatic storage).
  alignas(64) unsigned char local[64];
  if (!aligned_to(local, 64)) return 15;
  int* number = new (local) int(42);
  if (*number != 42) return 16;

  return live;
}
