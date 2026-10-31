// tidal-rules: INL-1
// tidal-expect: run
// tidal-sources: inline-variable-const-default-external.tu.cpp
inline const int shared = 6;
const int* other();
int main() { return &shared == other() ? 0 : 1; }
