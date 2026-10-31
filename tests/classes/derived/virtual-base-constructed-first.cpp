// tidal-rules: DER-7
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct V { V() { mark(1); } ~V() { mark(1); } };
struct B { B() { mark(2); } ~B() { mark(2); } };
struct C : virtual V { C() { mark(3); } ~C() { mark(3); } };
struct D : B, C { D() { mark(4); } ~D() { mark(4); } };
int main() { { D d; } return trace == 12344321 ? 0 : 1; }
