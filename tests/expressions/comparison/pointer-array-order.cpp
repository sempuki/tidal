// tidal-rules: CMP-7
// tidal-expect: accept
int arr[4];
static_assert(&arr[0] < &arr[3]);
static_assert(&arr[2] >= &arr[1]);
static_assert(arr + 4 > arr);
