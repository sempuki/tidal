# 0. Provenance

Tidal is made with deep respect for copyright, and for the years of
careful work that the many contributors to the ISO C++ standard have
given it.

**Tidal holds no text of the ISO standard, and rejects text that is
similar to it.** On every change, a tool compares the pages of the Tidal
specification with the text of the ISO standard, and a similar passage
fails the change. The same tool checks each summary that a source
reader writes, before a clean-room writer reads it.

## Definitions

This chapter names two specifications and two kinds of program. Each
term below means one thing only.

| Term | Meaning |
|---|---|
| **ISO standard** | ISO/IEC 14882:2024, the international standard that defines C++23, with its drafts. Tidal does not copy it, and the agents that write Tidal do not read it. |
| **Tidal specification** | The pages of this site: chapters 1 to 8, the glossary, and the rule index. Their source is the directory `specification/` of the Tidal repository. |
| **Baseline compilers** | Clang, GCC, and EDG, the compilers that {ref}`the front page <baseline-compilers>` lists. Tidal runs them as programs and records what they do. |
| **Tidal front ends** | The programs in the directory `frontend/` of the Tidal repository. Each one reads a C++ program and decides what the Tidal specification says about it. `frontend/next/` holds the newer of the two. |

The Tidal specification describes the same language as the ISO
standard, so that a program means the same under both. The ISO
standard remains the normative definition of C++. Tidal does not
replace it.

## Roles

| Agent | Forbidden | Reads | Writes |
|---|---|---|---|
| **Source reader** | Writing or changing any file of the repository; quoting a source or proposing wording | The ISO standard, WG21 papers, cppreference, compiler sources under restrictive licenses, and posts that quote any of these | Summaries in its own words, which describe behavior, gaps, and doubts |
| **Clean-room writer** | Reading protected material | Source readers' summaries that passed the non-similarity check, the behavior and diagnostics of the baseline compilers, sanitized compiler sources under permissive licenses, and every file of the repository | The Tidal specification, the tests, the tools, and the documentation |
| **Clean-room reader** | Reading anything other than Tidal's own files | The Tidal specification, the tests, and their results | Its own exercise directories, and the Tidal front ends |

An agent that reads protected material becomes a source reader for the
rest of its session. It cannot work in the clean room again in that
session.

A source reader's summary describes language behavior, gaps, and
doubts. It contains no code, no quoted or paraphrased text, no proposed
wording, and no section numbers from a source. Before a clean-room
writer reads a summary, the tool `tools/similarity.py` compares the
summary with a reference corpus. A summary with a flagged passage goes
back to the source reader to be reworded.

## Workflow

```{raw} html
<figure class="tidal-diagram">
<svg font-family="sans-serif" viewBox="0 0 840 480" role="img" aria-labelledby="wf-title wf-desc">
  <title id="wf-title">The Tidal workflow</title>
  <desc id="wf-desc">The clean room is on the left. The numbers are the steps of the pipeline listed below the diagram. 1: a clean-room writer picks the topic. 2: the writer probes the baseline compilers for their behavior and diagnostics. 3: the writer reads the source readers' summaries that passed the non-similarity check. Outside the clean room, source readers read the ISO standard and other protected sources. 4: the writer classifies the behavior. 5 and 6: the writer writes the rules, tests, and questions. 7: the non-similarity check compares the page with the reference corpus. 8: the tests run on the compilers, and the results become each page's Evidence. Clean-room readers, including the Tidal front ends, read only Tidal's files and send findings back to the writers.</desc>
  <defs>
    <marker id="wf-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
      <path d="M0,0 L10,5 L0,10 z" class="wf-head" fill="currentColor"/>
    </marker>
  </defs>
  <!-- the clean-room boundary -->
  <line x1="622" y1="28" x2="622" y2="470" class="wf-boundary" stroke="currentColor" stroke-dasharray="6 5"/>
  <text x="20" y="18" class="wf-note" fill="currentColor" font-size="11" font-style="italic" text-anchor="start">inside the clean room</text>
  <text x="636" y="18" class="wf-note" fill="currentColor" font-size="11" font-style="italic" text-anchor="start">outside the clean room</text>

  <!-- boxes -->
  <rect x="240" y="30" width="200" height="60" rx="6" class="wf-box" fill="none" stroke="currentColor" stroke-width="1.2"/>
  <text x="340" y="56" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">Baseline compilers</text>
  <text x="340" y="75" class="wf-label" fill="currentColor" font-size="13" text-anchor="middle">Clang, GCC, EDG</text>
  <rect x="240" y="190" width="200" height="60" rx="6" class="wf-box wf-accent" fill="none" stroke="currentColor" stroke-width="1.2"/>
  <text x="340" y="225" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">Clean-room writers</text>
  <rect x="240" y="340" width="200" height="70" rx="6" class="wf-box wf-accent" fill="none" stroke="currentColor" stroke-width="1.2"/>
  <text x="340" y="368" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">The Tidal specification</text>
  <text x="340" y="388" class="wf-label" fill="currentColor" font-size="13" text-anchor="middle">and its tests</text>
  <rect x="30" y="340" width="170" height="70" rx="6" class="wf-box" fill="none" stroke="currentColor" stroke-width="1.2"/>
  <text x="115" y="364" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">Clean-room readers</text>
  <text x="115" y="383" class="wf-label" fill="currentColor" font-size="13" text-anchor="middle">exercises and</text>
  <text x="115" y="400" class="wf-label" fill="currentColor" font-size="13" text-anchor="middle">the Tidal front ends</text>
  <rect x="470" y="190" width="130" height="60" rx="6" class="wf-box" fill="none" stroke="currentColor" stroke-width="1.2"/>
  <text x="535" y="207" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">Non-similarity</text>
  <text x="535" y="223" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">check</text>
  <text x="535" y="241" class="wf-label wf-mono" fill="currentColor" font-size="13" text-anchor="middle">tools/similarity.py</text>
  <rect x="640" y="190" width="180" height="60" rx="6" class="wf-box" fill="none" stroke="currentColor" stroke-width="1.2"/>
  <text x="730" y="225" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">Source readers</text>
  <rect x="640" y="40" width="180" height="76" rx="6" class="wf-box wf-protected" fill="none" stroke="currentColor" stroke-width="1.2" stroke-dasharray="5 4"/>
  <text x="730" y="66" class="wf-label" fill="currentColor" font-size="13" text-anchor="middle">The ISO standard,</text>
  <text x="730" y="84" class="wf-label" fill="currentColor" font-size="13" text-anchor="middle">WG21 papers, other</text>
  <text x="730" y="102" class="wf-label" fill="currentColor" font-size="13" text-anchor="middle">protected sources</text>

  <!-- steps 1 and 4: the writers pick the topic and classify -->
  <rect x="198.0" y="189" width="36" height="18" rx="9" class="wf-badge" fill="currentColor"/>
  <text x="216" y="202" class="wf-badge-text" fill="white" font-size="11" font-weight="bold" text-anchor="middle">1, 4</text>

  <!-- step 2: probe -->
  <line x1="340" y1="90" x2="340" y2="188" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <rect x="349.0" y="122" width="18" height="18" rx="9" class="wf-badge" fill="currentColor"/>
  <text x="358" y="135" class="wf-badge-text" fill="white" font-size="11" font-weight="bold" text-anchor="middle">2</text>
  <text x="372" y="135" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">behavior and</text>
  <text x="372" y="151" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">diagnostics</text>

  <!-- steps 5 and 6: draft and questions -->
  <line x1="340" y1="250" x2="340" y2="338" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <rect x="347.0" y="277" width="36" height="18" rx="9" class="wf-badge" fill="currentColor"/>
  <text x="365" y="290" class="wf-badge-text" fill="white" font-size="11" font-weight="bold" text-anchor="middle">5, 6</text>
  <text x="386" y="290" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">rules, tests,</text>
  <text x="386" y="306" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">questions</text>

  <!-- step 7: the check of the page -->
  <path d="M440,392 L535,392 L535,252" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <rect x="544.0" y="313" width="18" height="18" rx="9" class="wf-badge" fill="currentColor"/>
  <text x="553" y="326" class="wf-badge-text" fill="white" font-size="11" font-weight="bold" text-anchor="middle">7</text>
  <text x="567" y="326" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">pages</text>

  <!-- step 8: evidence -->
  <path d="M340,410 L340,440 L14,440 L14,60 L238,60" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <rect x="21.0" y="445" width="18" height="18" rx="9" class="wf-badge" fill="currentColor"/>
  <text x="30" y="458" class="wf-badge-text" fill="white" font-size="11" font-weight="bold" text-anchor="middle">8</text>
  <text x="46" y="458" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">tests run on the compilers; results go to each page's Evidence</text>

  <!-- findings -->
  <line x1="240" y1="375" x2="202" y2="375" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <path d="M115,340 L115,220 L238,220" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <text x="125" y="270" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">findings: gaps,</text>
  <text x="125" y="286" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">doubts, failures</text>

  <!-- step 3: summaries from outside -->
  <line x1="730" y1="116" x2="730" y2="188" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <text x="738" y="157" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">read</text>
  <line x1="640" y1="220" x2="602" y2="220" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <text x="552" y="180" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">summaries</text>
  <line x1="470" y1="220" x2="442" y2="220" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <rect x="447.0" y="189" width="18" height="18" rx="9" class="wf-badge" fill="currentColor"/>
  <text x="456" y="202" class="wf-badge-text" fill="white" font-size="11" font-weight="bold" text-anchor="middle">3</text>
  <text x="456" y="270" class="wf-edge" fill="currentColor" font-size="12" text-anchor="middle">passed</text>
</svg>
<figcaption>The numbers are the steps of the pipeline in the list
below. One badge can mark more than one step. The findings of
clean-room readers are not a step of the pipeline. They come back to the
writers at any time. Everything inside the clean room comes from the
baseline compilers, from summaries that passed the non-similarity check,
and from Tidal's own files.</figcaption>
</figure>
```

A page of the Tidal specification goes through these steps
(`documentation/pipeline.md` in the repository gives each one in
full):

1. **Topic.** A clean-room writer picks the language feature that the
   page will cover.
2. **Probe.** The writer runs small programs on the baseline compilers
   and records what each compiler accepts, rejects, and prints.
3. **Read sources.** The writer reads the sanitized, permissively
   licensed compiler sources where they help, and the source readers'
   summaries that passed the non-similarity check.
4. **Classify.** The writer sorts the behavior into kinds: required,
   rejected, undefined, unspecified, implementation-defined.
5. **Draft.** The writer writes the page's principles and rules. Each
   rule has an ID, a kind, and test programs that show it.
6. **Questions.** Where the compilers disagree, the writer adopts a
   provisional answer, records it as a question, and goes on.
7. **Non-similarity check.** `tools/similarity.py` compares the page with
   the reference corpus, and the writer rewrites each flagged passage.
8. **Evidence.** Every test runs on every baseline compiler, and the
   results go into the Evidence segment of the page.

The repository's history records each step. Each step is one commit,
pushed when it is made, and the history is never rewritten.

## Evolution

The Tidal specification improves through checks that never use the
text of the ISO standard. Each check finds a fault in a specific rule.

**The baseline compilers decide the behavior.** A rule of the Tidal
specification states what the baseline compilers do. Where they agree,
the rule records the agreement, and its tests show it. Where they
disagree, a question in the repository records the split and the answer
that Tidal adopts.

**The tests tie every rule to programs.** Each rule cites tests, and
each test states its expected outcome: accepted, rejected with a named
rule, run to a result, or stopped by undefined behavior. When a
compiler's result differs from the expected outcome, the rule or the
test is wrong.

**Clean-room readers use the text as its audience would.** A
clean-room reader writes programs from the Tidal specification alone,
predicts the outcomes of tests, or writes a guide to a task. A wrong
prediction, or a program that the reader cannot write, points to a rule
that is missing or unclear. The finding goes to the clean-room writers,
who change the wording.

**The Tidal front ends test the rules as code.** Clean-room readers
write the Tidal front ends from the Tidal specification and the Tidal
tests. They do not read the ISO standard or any compiler source. Each
rule of the Tidal specification becomes code that decides real
programs, and the front end runs on every test. Where a front end
cannot decide a program, or decides it against a test's expected
outcome, either the code is wrong or the rule is unclear or incomplete.
Vague wording cannot pass this check. The front end also lets a writer
try a change of wording against the whole test suite at once.

**Source readers add coverage, not text.** A source reader can report
that the Tidal specification misses a case, or that a rule is too
broad. The report passes the non-similarity check, and a clean-room writer
then probes the compilers and writes the rule independently.

**Closeness to the ISO standard is measured.** Tidal is also an
experiment in how close independent text ends up to the wording of the
ISO standard. The non-similarity checks that guard each step also measure
that closeness.
