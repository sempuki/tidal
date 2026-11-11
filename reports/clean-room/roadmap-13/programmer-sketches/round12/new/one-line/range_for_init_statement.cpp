// A range-based for with an init statement (C++20).
int main() { int a[] = {4, 5, 6}; int s = 0; for (int i = 0; int x : a) s += x * i++; return s - 17; }
