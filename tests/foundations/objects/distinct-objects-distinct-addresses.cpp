// tidal-rules: OBJ-7
// tidal-expect: run
struct E {};
int main() { int a, b; E e, f; E arr[2]; return &a != &b && &e != &f && &arr[0] != &arr[1] && sizeof(E) == 1 ? 0 : 1; }
