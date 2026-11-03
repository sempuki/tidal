// tidal-rules: ENUM-6
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: load of value 5, which is not a valid value
// tidal-note: Q-0336. The value 5 comes from the bytes of an int, with no conversion.
// tidal-note: Run-time check with the sanitizer (decision 0028).
enum Color { red, green, blue };  // holds the values 0 to 3
int main(int argc, char**) {
  int source = 4 + argc;          // 5
  Color c = red;
  unsigned char* to = reinterpret_cast<unsigned char*>(&c);
  const unsigned char* from = reinterpret_cast<const unsigned char*>(&source);
  for (decltype(sizeof 0) i = 0; i != sizeof c; ++i) to[i] = from[i];
  volatile Color v = c;           // the read of the stored value
  return v == red;
}
