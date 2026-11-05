// tidal-rules: STG-12 LSUP-9 STG-14
// tidal-expect: run
#include "byte.h"
int main() {
  double value = 6.25;
  char plain[sizeof value];
  std::byte typed[sizeof value];
  char* source = reinterpret_cast<char*>(&value);
  for (unsigned i = 0; i != sizeof value; ++i) {
    plain[i] = source[i];
    typed[i] = static_cast<std::byte>(source[i]);
  }
  value = 0.0;
  for (unsigned i = 0; i != sizeof value; ++i) source[i] = plain[i];
  bool from_plain = value == 6.25;
  value = 0.0;
  std::byte* target = reinterpret_cast<std::byte*>(&value);
  for (unsigned i = 0; i != sizeof value; ++i) target[i] = typed[i];
  return from_plain && value == 6.25 ? 0 : 1;
}
