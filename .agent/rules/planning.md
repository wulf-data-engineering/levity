---
trigger: always_on
---

# Planning Workflow

Before implementation check if the feature would make an existing adr obsolete or if a new adr is needed.
Suggest changes or a new adr and ask user for feedback.

## CRITICAL: Plan First

If the user asks for a _plan_, **DO NOT** modify any files yet. Other agents might be planning in parallel.
Only modify files after the user approves the plan and you switch to execution mode.
