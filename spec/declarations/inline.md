# The inline specifier

`inline` lets a program define a function or variable in each translation
unit that uses it. All these definitions are one entity. This page covers
where `inline` can occur, which declarations are inline without the
keyword, inline static data members, and inline functions across
translation units.

Inline namespaces are with namespaces. LNK-4 gives the rule for one
entity with one address.

Depends on: storage duration and linkage, function definitions, class
definitions and members.
