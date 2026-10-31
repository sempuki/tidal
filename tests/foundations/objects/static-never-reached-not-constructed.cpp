// tidal-rules: OBJ-4
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };

void never() { static T s(7); }
int main() { return trace == 0 ? 0 : 1; }
