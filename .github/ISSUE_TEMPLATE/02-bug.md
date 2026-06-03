---
name: "\U0001F41B Bug Report"
description: Report a bug or unexpected behavior
title: "[Bug] <short description>"
labels: ["type:bug"]
assignees: []
body:
  - type: textarea
    id: description
    attributes:
      label: Describe the Bug
      description: Clear and concise description of the issue.
    validations:
      required: true

  - type: textarea
    id: reproduction
    attributes:
      label: Steps to Reproduce
      placeholder: |
        1. Go to '...'
        2. Click on '...'
        3. See error
    validations:
      required: true

  - type: textarea
    id: expected
    attributes:
      label: Expected Behavior
      description: What should happen instead.
    validations:
      required: true

  - type: textarea
    id: environment
    attributes:
      label: Environment
      description: OS, app version, build type.
      placeholder: |
        - OS: macOS 26.x
        - App version: 0.1.0
        - Build: debug / release
    validations:
      required: false

  - type: textarea
    id: logs
    attributes:
      label: Logs / Screenshots
      description: Console output, crash logs, or screenshots.
    validations:
      required: false
