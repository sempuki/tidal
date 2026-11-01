// tidal-rules: SINIT-5
// tidal-expect: run
// tidal-sources: unit-not-named.tu.cpp
// tidal-note: main names nothing from the second unit.
int flag = 0;
int main() { return flag == 1 ? 0 : 1; }
