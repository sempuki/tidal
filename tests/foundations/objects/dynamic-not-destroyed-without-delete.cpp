// tidal-rules: OBJ-5
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };
int main() { new T(1); return trace == 1 ? 0 : 1; }
