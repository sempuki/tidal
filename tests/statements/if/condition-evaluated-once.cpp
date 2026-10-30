// tidal-rules: IF-1
// tidal-expect: run
int calls = 0;
bool check() { ++calls; return false; }
int main() {
  if (check()) {} else {}
  return calls == 1 ? 0 : 1;
}
