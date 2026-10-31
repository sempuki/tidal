// tidal-rules: NEW-5
// tidal-expect: run-fail
// tidal-flags: -fsanitize=address
// tidal-error: AddressSanitizer: attempting double-free
// tidal-note: Run-time check with the sanitizer (decision 0028).
int main(int argc, char**) {
  int* p = new int(argc);
  delete p;
  delete p;
}
