// tidal-rules: OBJ-3
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };
int f() { T a(1); T b(2); return 0; mark(9); }
int main() { f(); return trace == 1221 ? 0 : 1; }
