// tidal-rules: OBJ-4
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };
thread_local T tl(4);
int main() { int id = tl.id; return id == 4 && trace == 4 ? 0 : 1; }
