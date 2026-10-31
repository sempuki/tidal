// tidal-rules: OBJ-5
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };

T* make() { T* p = new T(1); mark(2); return p; }
int main() { T* p = make(); mark(3); delete p; return trace == 1231 ? 0 : 1; }
