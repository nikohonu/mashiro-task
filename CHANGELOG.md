# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Week recurrence for "do" command.

## [0.5.0] - 2024-04-03

### Changed
- Remove some code to simplify program logic.

## [0.4.0] - 2023-11-26

### Added

- Show how many times the task has been done.
- Task efficiency = (task completions / (today - created)) * recurrence.
- Option for the "now" command to choose a random task.
- "do" command can invoke the "now" command with the -n argument of the "do" command.

## [0.3.0] - 2023-11-22

### Added

- More fields to the "now" command table.
- Compact view for the "now" command.

### Changed

- New random optional task algorithm based on projects.
- Maintain the order of tasks when executing the "now" command again.

## [0.2.1] - 2023-11-22

### Fixed

- Tasks that had a now_date earlier than today were not added to the relevant tasks.

## [0.2.0] - 2023-11-19

### Added

- "add" command for creating new tasks.
- "completion" command to generate shell completion scripts.
- "now" command to display active tasks.
- "do" command for marking a task as completed.
- "remove" command for deleting a task.
- "regenerate-ids" command to generate new IDs for all tasks.
