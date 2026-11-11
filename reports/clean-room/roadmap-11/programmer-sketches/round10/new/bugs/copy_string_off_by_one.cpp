// Planted bug: the buffer has room for the characters but not the terminator.
void copy(char* dst, const char* src) { while ((*dst++ = *src++)) {} }
int main() {
  struct { char name[5]; int guard; } s{{}, 7};
  copy(s.name, "hello");
  return s.guard == 7 ? 0 : 1;
}
