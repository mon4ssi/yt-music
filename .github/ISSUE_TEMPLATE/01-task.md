---
name: "\U0001F9F0 Task"
description: Implementation task ready for agentic execution
title: "[T] <short description>"
labels: ["type:task", "status:ready", "ai:ready"]
assignees: []
body:
  - type: markdown
    attributes:
      value: |
        ## Objective
        _What does this task achieve? Keep to one sentence._

  - type: textarea
    id: scope
    attributes:
      label: In Scope
      description: Bullet list of exactly what this task covers.
      placeholder: |
        - Item 1
        - Item 2
    validations:
      required: true

  - type: textarea
    id: out-of-scope
    attributes:
      label: Out of Scope
      description: Bullet list of what this task explicitly does not cover.
      placeholder: |
        - Item 1
        - Item 2
    validations:
      required: false

  - type: textarea
    id: constraints
    attributes:
      label: Constraints
      description: Technical constraints, stack requirements, or patterns to follow.
      placeholder: |
        - Stack: Tauri v2 + React 19 + TypeScript
        - Follow existing naming and module conventions
        - No secrets or hardcoded credentials
    validations:
      required: false

  - type: textarea
    id: acceptance
    attributes:
      label: Acceptance Criteria
      description: Measurable checks that define done.
      placeholder: |
        - [ ] Criterion 1
        - [ ] Criterion 2
    validations:
      required: true

  - type: textarea
    id: verification
    attributes:
      label: Verification Commands
      description: Commands to run before marking complete.
      render: shell
      placeholder: |
        pnpm lint
        pnpm typecheck
        pnpm test
    validations:
      required: false

  - type: textarea
    id: notes
    attributes:
      label: Additional Notes
      description: Context, references, or related issues.
    validations:
      required: false
