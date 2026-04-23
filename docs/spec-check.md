# TodoMVC Spec Conformance Checklist

Last reviewed: 2026-04-23

This checklist records the current TodoMVC conformance status of the browser app, not just the presence of backend handlers or standalone Leptos components.

## Test Basis

- Reviewed the running app composition in [src/app.rs](/workspace/src/app.rs).
- Reviewed route behavior in [src/routes.rs](/workspace/src/routes.rs).
- Reviewed available UI components in [src/components/header.rs](/workspace/src/components/header.rs), [src/components/footer.rs](/workspace/src/components/footer.rs), [src/components/todo_list.rs](/workspace/src/components/todo_list.rs), and [src/components/todo_item.rs](/workspace/src/components/todo_item.rs).
- Reviewed server-side behavior in [src/server_fns.rs](/workspace/src/server_fns.rs) and the REST API tests in [tests/api.rs](/workspace/tests/api.rs).

## Current Summary

- Backend and server-function support exists for add, toggle, edit, delete, toggle-all, and clear-completed.
- The browser app shell does not yet mount the TodoMVC header/footer/list components or load todos from the server.
- Because of that, the current browser experience is not TodoMVC-conformant even though several lower-level pieces already exist.

## Checklist

| Requirement | Expected behavior | Status | Notes |
| --- | --- | --- | --- |
| Add todo | Entering text and pressing Enter creates a todo and renders it immediately | Fail | [src/app.rs](/workspace/src/app.rs) renders a static `<input>` and does not mount the real header component or todo list |
| Toggle completion | Clicking a todo checkbox toggles completed state and updates styling/counts | Fail | [src/components/todo_item.rs](/workspace/src/components/todo_item.rs) supports this, but the running app never renders `TodoItem` |
| Edit todo | Double-click enters edit mode; Enter saves; Escape cancels; empty save deletes | Fail | Implemented in [src/components/todo_item.rs](/workspace/src/components/todo_item.rs), but not reachable from the mounted app |
| Delete todo | Clicking the destroy button removes the todo | Fail | Implemented in [src/components/todo_item.rs](/workspace/src/components/todo_item.rs), but not reachable from the mounted app |
| Filter routes | `/`, `/active`, and `/completed` switch visible todo subsets and selected filter state | Partial | [src/routes.rs](/workspace/src/routes.rs) provides the routes, but they only render a placeholder label rather than a filtered todo list |
| Clear completed | Footer button removes completed todos | Fail | Implemented in [src/components/footer.rs](/workspace/src/components/footer.rs), but footer is not mounted by [src/app.rs](/workspace/src/app.rs) |
| Toggle all | Header toggle-all checkbox marks all todos complete/incomplete | Fail | Implemented in [src/components/header.rs](/workspace/src/components/header.rs), but header component is not mounted by [src/app.rs](/workspace/src/app.rs) |
| Count pluralization | Footer shows `1 item left` vs `N items left` correctly | Fail | Pluralization logic exists in [src/components/footer.rs](/workspace/src/components/footer.rs), but the footer is not mounted |
| Reload persistence | Todos survive a full browser reload because state is loaded from Postgres | Fail | The backend persists todos, but the browser app does not currently fetch and render persisted todos on load |

## Manual Validation Steps

Use these steps once the main app mounts the real components:

1. Open `/`.
2. Add two todos and confirm both appear.
3. Toggle one todo completed and verify styling plus active count update.
4. Double-click a todo label, edit it, save with Enter, then repeat and cancel with Escape.
5. Edit a todo to an empty string and confirm it is deleted.
6. Delete a todo via the destroy button.
7. Visit `/active` and `/completed` directly and confirm route-driven filtering plus selected filter styling.
8. Use `Clear completed` and verify only completed todos are removed.
9. Use the toggle-all checkbox to mark all complete, then all active again.
10. Verify footer text shows `1 item left` for one active todo and `2 items left` for two.
11. Reload the page and confirm the same todos are rendered from persisted storage.

## Blocking Gaps To Resolve Before Re-Check

- Replace the placeholder shell in [src/app.rs](/workspace/src/app.rs) with the actual TodoMVC component tree.
- Load todos into a `Resource` and pass invalidation callbacks through the mounted components.
- Mount the footer with live active/completed counts and route-aware filtering.
- Confirm persisted todos are fetched on initial page load after hydration and after a full reload.
