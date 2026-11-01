// tidal-rules: ICS-6 OVL-35
// tidal-expect: reject
// tidal-error-clang: cannot pass initializer list to variadic function
// tidal-error-gcc: invalid use of brace-enclosed initializer list
// tidal-error-edg: brace-enclosed list cannot be passed for an ellipsis parameter
// tidal-note: A braced list has no ellipsis sequence. Item has no conversion to Label, so neither function is viable.
struct Label { Label(int); };
struct Item {};
int show(Label);
long show(...);
void use(Item item) { show({item}); }
