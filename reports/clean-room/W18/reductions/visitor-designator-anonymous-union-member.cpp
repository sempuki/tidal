struct S { int k; union { int i; float f; }; };
int main() { S s{.k = 0, .f = 1.0f}; return s.k; }
