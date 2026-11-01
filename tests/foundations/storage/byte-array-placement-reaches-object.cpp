// tidal-rules: STG-29 STG-5 STG-8
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
// tidal-note: The program reaches an object in a byte array through the pointer that placement new returns, and not through a cast of the array's address (Q-0324).
#include "placement.h"
struct Registers { unsigned status; unsigned data; };
alignas(Registers) unsigned char device[2 * sizeof(Registers)];
int main() {
  Registers* bank = new (device) Registers[2]{{1, 2}, {3, 4}};
  volatile Registers* port = bank + 1;
  port->data = 9;
  bool same = static_cast<void*>(bank) == static_cast<void*>(device);
  return same && bank[0].status == 1 && bank[1].data == 9 ? 0 : 1;
}
