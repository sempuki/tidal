// A view that outlives the array that it points into.
struct View { const int* p; int n; };
View make() { int a[3] = {1, 2, 3}; return View{a, 3}; }
int main() { View v = make(); int s = 0; for (int i = 0; i < v.n; ++i) s += v.p[i]; return s == 6 ? 0 : 1; }
