// tidal-rules: DCL-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int* arr_of_ptr[3];
int (*ptr_to_arr)[3];
int (*(*fn_ptr_returning_arr_ptr)(int))[3];
int (*(*fp_returning_fp)(char))(double);
static_assert(same<decltype(arr_of_ptr), int* [3]>);
static_assert(same<decltype(ptr_to_arr), int (*)[3]>);
static_assert(same<decltype(fn_ptr_returning_arr_ptr), int (*(*)(int))[3]>);
using Inner = int (*)(double);
static_assert(same<decltype(fp_returning_fp), Inner (*)(char)>);
