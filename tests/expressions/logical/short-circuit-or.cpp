// tidal-rules: LOG-4
// tidal-expect: run
int calls = 0;
bool count() { ++calls; return false; }
int main() {
  bool r = true || count();
  bool s = false || count();
  return r && !s && calls == 1 ? 0 : 1;
}
