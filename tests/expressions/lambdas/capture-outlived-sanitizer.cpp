// tidal-rules: LAM-19
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address
// tidal-error: AddressSanitizer: stack-use-after-scope
// tidal-note: Run-time check with the sanitizer (decision 0028).
int main(int argc, char**) {
  auto make = [](int& r) { return [&r] { return r; }; };
  decltype(make(argc))* get;
  {
    int inner = argc;
    get = new auto(make(inner));
  }
  int value = (*get)();
  delete get;
  return value;
}
