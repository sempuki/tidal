// tidal-rules: DED-13 DED-35
// tidal-expect: reject
// tidal-error-clang: couldn't infer template argument
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: The parameter int[N] becomes int*, so the bound N is lost.
template <int N> void fill(int values[N]);
int arr[3];
void use() { fill(arr); }
