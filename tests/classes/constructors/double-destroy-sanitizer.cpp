// tidal-rules: DTOR-7
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address
// tidal-error: AddressSanitizer: attempting double-free
// tidal-note: Run-time check with the sanitizer (decision 0028). The end of the block destroys owner a second time, and the second destructor run deletes held again.
struct Owner {
  int* held;
  Owner() : held(new int(1)) {}
  ~Owner() { delete held; }
};
int main() {
  Owner owner;
  owner.~Owner();
  return 0;   // undefined: the block's end destroys owner again
}
