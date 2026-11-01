// tidal-rules: STG-14
// tidal-expect: run
#include "byte.h"
int through_char(int* number, char* other) { *number = 1; *other = 0; return *number; }
int through_unsigned_char(int* number, unsigned char* other) { *number = 1; *other = 0; return *number; }
int through_byte(int* number, std::byte* other) { *number = 1; *other = std::byte{0}; return *number; }
int main() {
  int (*volatile a)(int*, char*) = through_char;
  int (*volatile b)(int*, unsigned char*) = through_unsigned_char;
  int (*volatile c)(int*, std::byte*) = through_byte;
  int value = 0;
  int r = a(&value, reinterpret_cast<char*>(&value));
  r += b(&value, reinterpret_cast<unsigned char*>(&value));
  r += c(&value, reinterpret_cast<std::byte*>(&value));
  return r == 0 ? 0 : 1;   // each write of a zero byte reaches the int
}
