// tidal-rules: TPL-45
// tidal-expect: run
// tidal-note: The plain function twice(int) and the specialization twice<int> have the same name and type. They are two functions with their own bodies.
template <class T> int twice(T) { return 1; }
int twice(int) { return 2; }
int main() { return twice(3) == 2 && twice<int>(3) == 1 ? 0 : 1; }
