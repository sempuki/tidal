// tidal-rules: NEW-5 CPY-4
// tidal-expect: run-fail
// tidal-flags: -fsanitize=address
// tidal-error: AddressSanitizer: attempting double-free
// tidal-note: Run-time check with the sanitizer (decision 0028).
struct Owner {
  int* items;
  explicit Owner(int n) : items(new int[n]()) {}
  ~Owner() { delete[] items; }
};
int main(int argc, char**) {
  Owner a(argc);
  Owner b = a;
  return b.items[0];
}
