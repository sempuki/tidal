// tidal-rules: OBJ-3
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };
int main() { { T a(1); { T b(2); } T c(3); } return trace == 122331 ? 0 : 1; }
