// tidal-rules: OBJ-3
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };
int main() { for (int i = 1; i <= 2; ++i) { T a(i); } return trace == 1122 ? 0 : 1; }
