// tidal-rules: ODR-2
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: reference to local variable .limit. declared in enclosing function
// tidal-error-gcc: use of local variable with automatic storage from containing function
// tidal-note: Q-0130. EDG accepts this use of a constant of the enclosing function.
int main() {
  const int limit = 4;
  struct Reader {
    int read() { const int& ref = limit; return ref; }
  };
  return Reader().read() - 4;
}
