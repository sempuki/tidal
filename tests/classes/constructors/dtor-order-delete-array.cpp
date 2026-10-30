// tidal-rules: DTOR-5
// tidal-expect: run
int trace = 0;
int next = 0;
struct T { int id = ++next; ~T() { trace = trace * 10 + id; } };
int main() { T* p = new T[3]; delete[] p; return trace == 321 ? 0 : 1; }
