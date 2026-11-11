// tidal-rules: TYN-10
// tidal-expect: reject
// tidal-error-clang: an attribute list cannot appear here
// tidal-expect-gcc: accept
// tidal-error-edg: attributes are not allowed here
// tidal-note: Ledger row RM12-2. An attribute after the class key belongs only to a forward declaration such as struct [[deprecated]] Old;. Here the elaborated type is part of a variable declaration. Clang and EDG reject the attribute. GCC accepts it (Q-0445).
struct Node;
struct [[maybe_unused]] Node* head = nullptr;
