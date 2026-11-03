// tidal-rules: PRIM-32
// tidal-expect: run
// tidal-note: On the matrix targets the letters have consecutive ASCII values, so a case change adds or subtracts 32 and 'z' - 'a' is 25.
char upper(char c) { return c >= 'a' && c <= 'z' ? static_cast<char>(c - 32) : c; }
int main() {
  char word[] = "tidal, c++23";
  for (char* p = word; *p != 0; ++p) *p = upper(*p);
  const char expected[] = "TIDAL, C++23";
  for (int i = 0; i < 13; ++i) {
    if (word[i] != expected[i]) return 1;
  }
  int span = 'z' - 'a';
  return span == 25 && 'Z' - 'A' == 25 && 'a' - 'A' == 32 ? 0 : 2;
}
