// tidal-rules: VOL-1
// tidal-expect: run
struct Regs { unsigned status; unsigned data; };
Regs bank{1, 0};                       // not volatile
int main() {
  volatile Regs* port = &bank;         // each access through port is observable
  port->data = 7;
  unsigned seen = port->status;
  return seen == 1 && bank.data == 7 ? 0 : 1;   // a plain read of the same object is valid
}
