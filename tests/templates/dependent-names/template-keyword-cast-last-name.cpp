// tidal-rules: DEP-25
// tidal-expect: accept
// tidal-note: RV10-4. The keyword template comes before the last template-id of the qualified type of a functional cast, with a braced list and with parentheses. Every compiler accepts both forms.
struct Parts { template <class U> struct Part { U value; }; };
static_assert(Parts::template Part<int>{4}.value == 4);
static_assert(Parts::template Part<int>().value == 0);
