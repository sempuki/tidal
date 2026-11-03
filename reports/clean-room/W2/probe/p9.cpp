int calls = 0;
int mark(int v) { ++calls; return v; }
int main() { int s = mark(1) << mark(2); (void)s; return calls; }
