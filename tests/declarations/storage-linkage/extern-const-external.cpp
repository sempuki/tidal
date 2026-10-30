// tidal-rules: LNK-3
// tidal-expect: run
// tidal-sources: extern-const-external.tu.cpp
extern const int answer;
int main() { return answer == 42 ? 0 : 1; }
