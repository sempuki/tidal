inline int shared_inline = 5;
inline int twice(int v) { return v * 2; }
const int* other_address() { return &shared_inline; }
int (*other_function())(int) { return &twice; }
