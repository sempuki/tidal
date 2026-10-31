// tidal-rules: VOL-1
// tidal-expect: run
// tidal-sources: const-volatile-external.tu.cpp
const volatile int shared = 7;
const volatile int* other();
int main() { return &shared == other() ? 0 : 1; }
