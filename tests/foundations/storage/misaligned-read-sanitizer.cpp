// tidal-rules: STG-23 ALN-4
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: load of misaligned address
// tidal-note: Run-time check with the sanitizer (decision 0028). No int sits at an odd address, so the read reaches a byte.
struct alignas(int) Buffer { unsigned char bytes[8]; };
int main(int argc, char**) {
  Buffer buffer{};
  int* p = reinterpret_cast<int*>(buffer.bytes + argc);   // offset 1
  return *p;
}
