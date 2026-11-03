// tidal-rules: OBJ-6
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };

struct W { T a{1}; T b{2}; ~W() { mark(9); } };
int main() { { W w; } return trace == 12921 ? 0 : 1; }
