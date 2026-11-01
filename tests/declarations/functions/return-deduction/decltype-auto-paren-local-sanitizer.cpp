// tidal-rules: RET-6 OBJ-2
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address
// tidal-error-clang: AddressSanitizer: stack-use-after-return
// tidal-error-gcc: AddressSanitizer: SEGV
// tidal-note: Run-time check with the sanitizer (decision 0028). Clang reports the use of the returned local. GCC returns a null reference for the address of a local, and the read faults.
// tidal-note: RET-6 is the specific rule, and OBJ-2 is the general rule that names it (decision 0052).
decltype(auto) copy_of(int n) {
  int value = n;
  return (value);   // int&&: a reference to value
}
int main(int argc, char**) {
  int&& result = copy_of(argc);
  return result == 1 ? 0 : 1;   // undefined: value no longer exists
}
