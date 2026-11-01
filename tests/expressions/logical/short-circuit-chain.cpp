// tidal-rules: LOG-4
// tidal-expect: run
int calls = 0;
bool yes() { ++calls; return true; }
bool no() { ++calls; return false; }
int main() {
  bool a = no() && yes() && yes();
  bool b = yes() || no() || no();
  bool c = no() || (yes() && no()) || yes();
  return !a && b && c && calls == 6 ? 0 : 1;
}
