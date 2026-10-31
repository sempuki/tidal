// tidal-rules: INL-1
// tidal-expect: run
// tidal-sources: constexpr-variable-internal.tu.cpp
constexpr int limit = 3;
const int* other();
int main() { return &limit != other() ? 0 : 1; }
