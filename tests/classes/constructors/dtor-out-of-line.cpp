// tidal-rules: DTOR-1
// tidal-expect: run
int count = 0;
struct S { ~S(); };
S::~S() { ++count; }
int main() { { S s; } return count == 1 ? 0 : 1; }
