# Decisions that wait on the maintainer

Each row is one item that waits on the maintainer (decision 0054). Tidal
keeps the answer in the third column until the maintainer decides. The
last column lists the questions that the decision closes or changes.

| # | Item | Options | Tidal keeps | Questions |
|---|---|---|---|---|
| 1 | The choices of decision 0054: settling by review, and program tests under `tests/programs/` | Confirm, change, or drop each choice | Both choices. A question settles by review only when at most one newest family differs from the answer in each form | Confirms the 34 questions settled on 2026-11-03 under decision 0054. If the maintainer lets the rule settle a question where two families differ, these 11 settle too: Q-0133, Q-0148, Q-0153, Q-0159, Q-0216, Q-0217, Q-0227, Q-0234, Q-0320, Q-0339, Q-0341 |
| 2 | The choices of decision 0051: settling when the newest version of each family agrees, and keeping the implementation-only operations referenced | Confirm or change each choice | Both choices | Q-0322 |
| 3 | A report of EDG behavior to the EDG project ([groups below](#edg-report)) | Approve the report, or hold it | Tidal follows Clang 22 and GCC 16. Each test records EDG's result as an override | Q-0002, Q-0003, and the cluster 5 groups below |
| 4 | The matrix agrees, and a gap review reads the C++23 text the other way | Follow the review, against every compiler, or follow the matrix | The matrix's answer in each, except Q-0307, where Tidal follows the review | Q-0300, Q-0181, Q-0243 (and the deduced form of Q-0226), Q-0275, Q-0277 (first form), Q-0280 (the two guide templates), Q-0225 (`T::box<int>` as an alias target without `typename`), Q-0307, Q-0336 (the read of an enumeration value that a byte copy stored, where both sanitizers report the read). Rules with no question, from the gap reviews of the exceptions chapter and names: THR-3 (a usable copy constructor when no copy runs), TRY-2 (a handler of a reference to a pointer that is not `const`), UNW-3 (the global deallocation form for a single object with extended alignment, where UNW-3 allows both forms), NAM-40 and NAM-41 (a class name in the type of a conversion function). From the gap reviews of the object model and program structure: STG-9 (a new object in the place of a member of a base subobject), LSUP-11 (an `std::allocator` whose element type is not a literal type), TR-P6 with TR-13, TR-14, and TR-15 (rejection of a program past a translation limit) |
| 5 | The variable form of Q-0235 | Reject the unqualified name (GCC 16), or accept it (Clang 22, EDG 7.0, and the cluster 6 review, with doubt) | Reject. INST-21 keeps one answer for the class, function, and variable forms | Q-0235 |
| 6 | Q-0343 and Q-0041 | Follow the gap review of item O, or follow the compilers | The review's reading. CPY-19 says a class with a deleted destructor, or with every copy and move function deleted, is not trivially copyable, and every family's builtin query says it is. JMP-6 accepts the jump, as GCC does, and Clang and EDG reject it | Q-0343, Q-0041 |
| 7 | A probe of which C++23 rejections later language versions accept, as an index with no rules (designer view, item 4) | Run the probe, or keep later versions out of scope | C++23 only. Newer features are rejected under strict mode | Q-0007. A choice to follow later changes also decides Q-0181 and Q-0188 |
| 8 | An outside reader for the Foundations chapter and two topic pages (decision 0024) | Ask the reviewer who prompted the principles-first change, or skip | No outside reading | None |
| 9 | Process questions that name no rule ([list below](#process-questions)) | Confirm or change each provisional answer | Each provisional answer | Q-0006, Q-0008, Q-0116, Q-0119, Q-0182, Q-0183, Q-0186, Q-0312 |

## EDG report

Clang 22 and GCC 16 agree, and EDG 7.0 differs, in these questions of
cluster 5. Questions marked "settled" settled under decision 0054, and
EDG's result stays a recorded deviation.

| Group | Questions |
|---|---|
| EDG rejects a program that Tidal accepts (13) | Q-0198, Q-0201, Q-0203, Q-0208 (settled), Q-0229 (settled), Q-0231 (settled), Q-0240 (settled), Q-0256 (settled), Q-0260 (settled), Q-0266, Q-0268 (settled), Q-0277, Q-0281 (settled) |
| EDG accepts a program that Tidal rejects (13) | Q-0146 (settled), Q-0170 (settled), Q-0172 (settled), Q-0224 (settled), Q-0237 (settled), Q-0245 (settled), Q-0265 (settled), Q-0267 (settled), Q-0272, Q-0278, Q-0280, Q-0286 (settled), Q-0292 (settled) |
| EDG follows another order or model of a step (3) | Q-0215 (settled), Q-0279 (settled), Q-0284 (settled) |
| EDG diagnoses a program that needs no diagnostic (1) | Q-0283 (settled) |
| Of the same kind, from cluster 3 (2) | Q-0214, Q-0285 (settled) |

The report leaves out four questions where the cluster 5 review reads
the C++23 text as EDG does:

- Q-0195 and Q-0197: TPL-35 and TPL-38 require a diagnostic, as EDG
  gives one. Clang 22 and GCC 16 accept the programs.
- Q-0235 and Q-0282: the review reads them as EDG does, with doubt.
  Tidal keeps the answer of Clang 22 and GCC 16. The maintainer can
  decide either question.

## Process questions

| Question | Tidal keeps |
|---|---|
| Q-0006: when a report compiler becomes a baseline compiler | After one clean review of its results |
| Q-0008: the similarity thresholds | An 8-word run and 0.25 overlap |
| Q-0116: a designer persona that is a source reader | The clean-room designer only |
| Q-0119: a diagram renderer | Plain text |
| Q-0182: reasons in Evidence | Only when a clean input states the reason |
| Q-0183: a cost model for proposals | None. A proposal states cost in words |
| Q-0186: judgments from outside Tidal in a proposal | Allowed when marked as outside knowledge |
| Q-0312: a list of token sequences that no production accepts | TR-12 gives the grammar its effect, and no rule lists such sequences. The maintainer decides whether the roadmap also keeps a list (option 3) |
