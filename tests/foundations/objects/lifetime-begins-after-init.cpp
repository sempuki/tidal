// tidal-rules: OBJ-1
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };

struct W { T a; T b; int seen; W() : a(1), b(2), seen(trace) { mark(3); } };
int main() { W w; return w.seen == 12 && trace == 123 ? 0 : 1; }
