// tidal-rules: UN-13
// tidal-expect: run
int calls = 0;
int count() { return ++calls; }
int main() {
  bool b = noexcept(count());
  return calls == 0 && !b ? 0 : 1;
}
