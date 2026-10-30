// tidal-rules: REF-1
// tidal-expect: accept
int x = 1;
int& a = x;
const int& b = x;
int arr[2];
int (&c)[2] = arr;
void f();
void (&d)() = f;
