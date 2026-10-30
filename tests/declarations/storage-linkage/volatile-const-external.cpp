// tidal-rules: LNK-3
// tidal-expect: run
// tidal-sources: volatile-const-external.tu.cpp
extern const volatile int signal_value;
int main() { return signal_value == 3 ? 0 : 1; }
