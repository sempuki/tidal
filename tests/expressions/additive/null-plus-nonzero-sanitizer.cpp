// tidal-rules: ADD-10
// tidal-expect: run-fail
// tidal-flags: -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-expect-gcc: run
// tidal-error-clang: non-zero offset 1 to null pointer
// tidal-note: The GCC sanitizer does not check an offset from a null pointer.
// tidal-note: Run-time check with the sanitizer (decision 0028).
int main(int argc, char**) {
  char* volatile p = nullptr;
  char* q = p + argc;
  return q == nullptr;
}
