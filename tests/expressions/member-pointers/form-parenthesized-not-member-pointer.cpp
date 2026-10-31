// tidal-rules: MPTR-2
// tidal-expect: run
struct T { int v = 5; int* get() { return &(T::v); } };
int main() { T t; return *t.get() == 5 ? 0 : 1; }
