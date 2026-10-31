// tidal-rules: OBJ-11
// tidal-expect: run
// tidal-note: Clang 18 and 22 and GCC 16 in the containers construct the object at its first use. The host compilers and GCC 14 construct it before main.
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };
thread_local T tl(4);
int main() { int before = trace; int id = tl.id; return (before == 0 || before == 4) && id == 4 && trace == 4 ? 0 : 1; }
