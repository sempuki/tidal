# I1 notes: steps the text did not give

Written in phase 1, before reading any test.

1. **Empty class.** No page defines an empty class. The glossary has no
   entry. Choice: a class that is not a union, has no non-static data
   member other than unnamed bit-fields of width 0, has no virtual
   function and no virtual base, and whose bases are all empty. A class
   whose only member is an empty `[[no_unique_address]]` member counts as
   not empty.
2. **Size of the pointer for virtual calls.** CLS-35 places the pointer
   but no rule gives its size or alignment. Choice: the size and
   alignment of a pointer (TYP-1, TYP-2).
3. **Size of a reference member and of a pointer to member.** Not given.
   Choice: the size of a pointer for both, and for a pointer to member
   function too.
4. **Alignment used to place a base with virtual bases.** CLS-35 says a
   base starts at a multiple of "its alignment". For a base whose class
   has virtual bases, the base subobject holds only part of the class.
   Choice: the full `alignof` of the class.
5. **Where an empty `[[no_unique_address]]` member goes.** CLS-35 sends
   an empty base to offset 0, but has no row for an empty member. CLS-31
   says such a member takes no bytes. Choice: the padding row ("each
   base and member starts at the end of the data so far, rounded up"),
   and the member adds nothing to the data so far. A later member can
   then start at a lower offset than the empty member.
6. **End of the data of a class.** The rows for bases and
   `[[no_unique_address]]` members say "the padding at the end of `C`"
   and "after the last member of `C`". Choice: the end of the last byte
   of the last base, member, or bit-field that holds data, before the
   class is rounded up to its alignment. Empty subobjects add nothing.
7. **Size of the base-subobject part of a class with virtual bases.**
   Needed only for the "covers every subobject" step. Choice: the end of
   the data of that part, or of the last empty subobject in it.
8. **Depth-first order of virtual bases.** "First meets" read as
   preorder: a virtual base is recorded when the walk reaches it, before
   the walk enters its own bases.
9. **Virtual bases that are empty.** The empty-base row says "an empty
   base", with no word on virtual bases. Choice: the row applies to
   virtual bases too, so an empty virtual base goes to offset 0 unless
   that offset conflicts.
10. **The conflict search for an empty base.** The empty base first
    tries offset 0, then the end of the data so far rounded up, then
    each next multiple of its alignment (CLS-35).
11. **Which alignment a named bit-field adds to the class.** BF-9 says
    an unnamed bit-field does not change the alignment of the class,
    and a wide one gives the class the alignment of `U`. Nothing says a
    named bit-field of normal width gives the class `alignof(T)`.
    Choice: a named bit-field adds `alignof(T)`.
12. **`U` in BF-9.** "The largest integer type with at most N bits".
    Choice: the ladder `char`, `short`, `int`, `long long` (sizes from
    TYP-1). `long` is the same size as `int` or `long long` on each
    target. The GCC 128-bit type is outside the model.
13. **"Starts at the next multiple" for a wide bit-field.** Read as
    rounding the current bit position up, so a position that is already
    a multiple stays.
14. **A class that keeps its end padding.** Implemented for GCC 16 and
    Clang (the default), and GCC 14 and EDG 7.0 by option. A union keeps
    its end padding by the same conditions (the text does not say that
    the conditions apply to unions; nothing excludes them). A reference
    member is not scalar, so a class with one does not keep it.
15. **A union.** No rule gives the size of a union directly. Choice from
    UNI-P1 and ALN-1: every member at offset 0, size the largest member
    size rounded up to the largest alignment, size at least 1.
16. **`long long` on `-m32`.** TYP-2 gives 4 for GCC and Clang and 8 for
    EDG. The default compiler is GCC 16, so 4, also for `alignof`
    outside a class ("also as a member of a class" read as "in every
    position").
17. **A class with its own virtual function and a dynamic base.** CLS-35
    says the class uses the pointer of the first such base. The other
    bases follow in base-list order with the primary one skipped.
