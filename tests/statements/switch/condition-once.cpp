// tidal-rules: SW-1
// tidal-expect: run
int calls = 0;
int value() { ++calls; return 2; }
int main() {
  switch (value()) { case 1: break; case 2: break; }
  return calls == 1 ? 0 : 1;
}
