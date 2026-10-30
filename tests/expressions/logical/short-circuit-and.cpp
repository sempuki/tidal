// tidal-rules: LOG-4
// tidal-expect: run
int calls = 0;
bool count() { ++calls; return true; }
int main() {
  bool r = false && count();
  bool s = true && count();
  return !r && s && calls == 1 ? 0 : 1;
}
