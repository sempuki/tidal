// tidal-rules: OBJ-8
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };

int main() { { const T& r = T(1); mark(2); } mark(3); return trace == 1213 ? 0 : 1; }
