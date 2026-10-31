// tidal-rules: OBJ-6
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };
int main() { { T a[3] = {1, 2, 3}; } return trace == 123321 ? 0 : 1; }
