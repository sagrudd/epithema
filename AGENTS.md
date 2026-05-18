# AGENTS.md

This file defines repository-local working rules for human and AI contributors.

## Operating Rules

1. Keep changes scoped.
   - Any code or documentation modification must touch only the parts of the
     codebase that are essential to the intended change.
   - Do not refactor unrelated modules as part of an otherwise local fix or
     feature.

2. Maintain git hygiene through completion.
   - Before starting work, inspect `git status`.
   - After making modifications, ensure the working tree is understood and
     intentional.
   - Any prompt that results in code changes must end with those changes being
     reviewed, committed, and pushed before the task is considered complete.
   - Documentation-only changes should also be committed and pushed unless a
     user explicitly asks to keep them local.

3. Maintain documentation honestly and religiously.
   - Documentation is a first-class subsystem.
   - Do not imply validation, provenance, harvesting, comparison, or feature
     completeness that does not exist.
   - Generated pages, autodoc contracts, validation stubs, and release/readiness
     reports must describe the actual state of the codebase.

4. Keep code method-associated and legible.
   - Rust implementation files should prefer method-associated naming where a
     tool has method-specific logic.
   - When logic spans more than one method, split it into discrete, logically
     named files rather than growing unrelated shared modules.
   - Prefer explicit naming over broad catch-all modules.

5. Preserve governed architecture.
   - The governed tool registry is the source of truth for exposed methods.
   - Documentation, validation, release reporting, and bridge surfaces should
     derive from governed sources rather than hand-maintained parallel lists.

6. Prefer additive, low-risk progress.
   - Stabilization and sweep work should proceed in coherent families.
   - Avoid speculative redesigns unless a task explicitly calls for them.

7. Validate before close-out.
   - Run the smallest practical set of checks that demonstrates the change is
     correct.
   - If environment limits block a check, state that plainly rather than
     implying success.
