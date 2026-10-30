// tidal-rules: STMT-1
// tidal-expect: run
int calls = 0;
int tick() { return ++calls; }
int main() {
  tick();
  tick() + 1;
  return calls == 2 ? 0 : 1;
}
