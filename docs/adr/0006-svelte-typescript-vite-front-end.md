# 0006. Svelte, TypeScript and Vite for the interface

Date: 2026-10-07
Status: Accepted

## Context

The first interface is plain HTML, CSS and JavaScript without a build step. The software has to
evolve and stay maintainable, which calls for types and components.

## Decision

- The interface moves to TypeScript and Svelte, built with Vite into static files embedded in
  the application. Nothing is loaded from outside at runtime.
- No SvelteKit: a single view needs no router or server features.
- The migration changes no function; it is done before any new interface work.

## Consequences

- The interface gains a build step and npm dependencies, each justified before being added, with
  versions locked.
- Type checking with `svelte-check` joins the mandatory checks.
