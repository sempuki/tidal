// W13 task 1: a device-register map over a block of memory, using volatile.
// Rules used: VOL-1, VOL-2, VOL-3, VOL-5, CV-9, TR-7, STG-4, STG-5,
// STG-13, STG-14, STG-26, NEW-33, LSUP-4, LSUP-7, ALN-1, CLS-10, TYP-1.

namespace std {
using size_t = decltype(sizeof 0);          // LSUP-4
}

// LSUP-7: the non-allocating placement forms, defined by the program.
inline void* operator new(std::size_t, void* at) noexcept { return at; }
inline void* operator new[](std::size_t, void* at) noexcept { return at; }
inline void operator delete(void*, void*) noexcept {}
inline void operator delete[](void*, void*) noexcept {}

// One bank of four 32-bit registers.
struct UartRegs {
  unsigned status;    // offset 0
  unsigned control;   // offset 4
  unsigned data;      // offset 8
  unsigned count;     // offset 12

  // VOL-2: volatile member functions can be called on a volatile object.
  bool ready() const volatile { return (status & 1u) != 0; }
  void start() volatile { control |= 1u; }   // VOL-5: compound assignment
};

static_assert(sizeof(unsigned) == 4);       // TYP-1
static_assert(sizeof(UartRegs) == 16);      // CLS-10, no padding expected
static_assert(alignof(UartRegs) == 4);

constexpr unsigned status_ready = 1u;
constexpr unsigned control_start = 1u;

// The block of memory that stands in for the device. STG-4: a byte array
// can host objects.
alignas(UartRegs) unsigned char device_memory[2 * sizeof(UartRegs)];

// The "hardware". It reacts to the control register.
void device_step(volatile UartRegs& r) {
  if ((r.control & control_start) != 0) {
    r.data = 0x41u + r.count;   // a new byte arrives
    r.count += 1u;              // VOL-5: not deprecated
    r.status |= status_ready;
    r.control &= ~control_start;
  }
}

// The driver. Each read of status happens again (VOL-1, TR-7).
unsigned receive(volatile UartRegs& r) {
  r.start();
  int spins = 0;
  while (!r.ready()) {
    device_step(r);
    if (++spins > 10) return 0xFFFFFFFFu;
  }
  unsigned value = r.data;      // the value is an ordinary unsigned
  r.status &= ~status_ready;
  return value;
}

int main() {
  // STG-5, NEW-33: create two register banks in the block and keep the
  // pointer that placement new returns. VOL-3 adds volatile.
  volatile UartRegs* const banks = new (device_memory) UartRegs[2]{};

  volatile UartRegs& uart0 = banks[0];
  volatile UartRegs& uart1 = banks[1];

  if (uart0.ready()) return 1;
  if (receive(uart0) != 0x41u) return 2;
  if (receive(uart0) != 0x42u) return 3;
  if (uart0.count != 2u) return 4;
  if (uart1.count != 0u) return 5;
  if (receive(uart1) != 0x41u) return 6;

  return 0;
}
