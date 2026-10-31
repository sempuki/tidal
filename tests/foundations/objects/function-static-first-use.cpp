// tidal-rules: OBJ-4
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };

T& get() { static T s(5); return s; }
int main() { mark(1); get(); get(); mark(2); return trace == 152 ? 0 : 1; }
