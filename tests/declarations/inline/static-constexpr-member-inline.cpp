// tidal-rules: INL-3
// tidal-expect: run
// tidal-sources: static-constexpr-member-inline.tu.cpp
struct S { static constexpr int limit = 7; };
const int* other();
int main() { return &S::limit == other() ? 0 : 1; }
