// tidal-rules: ENUM-6 CAST-14
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: load of value 5, which is not a valid value
// tidal-note: The sanitizer reports the read of the value, after the conversion.
// tidal-note: Run-time check with the sanitizer (decision 0028).
enum Color { red, green, blue };  // holds the values 0 to 3
int main(int argc, char**) {
  volatile Color c = static_cast<Color>(4 + argc);
  return c == red;
}
