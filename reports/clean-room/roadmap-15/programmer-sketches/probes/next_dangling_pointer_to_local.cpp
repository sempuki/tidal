const int* addr() { int x = 2; const int* p = &x; return p; }
int main() { const int* p = addr(); return *p == 2 ? 0 : 1; }
