// tidal-rules: OBJ-9
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };
int use(const T&) { mark(5); return 0; }
int main() { use(T(1)); mark(2); return trace == 1512 ? 0 : 1; }
