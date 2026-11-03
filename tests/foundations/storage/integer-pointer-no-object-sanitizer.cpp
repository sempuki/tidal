// tidal-rules: UN-7 STG-15
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address -fno-sanitize-recover=all
// tidal-error: SEGV on unknown address
// tidal-note: Q-0325. No object lies at the address, so the pointer is an invalid pointer value, and the read through it is undefined.
using address = decltype(sizeof 0);
int main() {
  auto* nowhere = reinterpret_cast<volatile unsigned*>(address{0x4000'1000});
  return *nowhere == 0 ? 2 : 3;
}
