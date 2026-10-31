struct S { static inline int count = 4; };
int* other() { return &S::count; }
