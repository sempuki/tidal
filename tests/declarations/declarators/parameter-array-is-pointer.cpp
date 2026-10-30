// tidal-rules: DCL-8
// tidal-expect: run
int size_of_param(int a[100]) { return sizeof(a) == sizeof(int*) ? 0 : 1; }
int main() { int x[100]; return size_of_param(x); }
