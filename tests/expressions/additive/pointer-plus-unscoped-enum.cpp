// tidal-rules: ADD-3
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31 (third).
enum E { one = 1 };
int arr[4];
int* p = arr + one;
int* q = arr + true;
int* r = arr + '\3';
