// tidal-rules: INL-4
// tidal-expect: run
// tidal-sources: inline-static-member.tu.cpp
struct S { static inline int count = 4; };
int* other();
int main() { ++S::count; return S::count == 5 && &S::count == other() ? 0 : 1; }
