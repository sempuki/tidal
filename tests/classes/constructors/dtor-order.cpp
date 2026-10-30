// tidal-rules: DTOR-5
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct B1 { ~B1() { mark(5); } };
struct B2 { ~B2() { mark(4); } };
struct M1 { ~M1() { mark(3); } };
struct M2 { ~M2() { mark(2); } };
struct S : B1, B2 { M1 a; M2 b; ~S() { mark(1); } };
int main() { { S s; } return trace == 12345 ? 0 : 1; }
