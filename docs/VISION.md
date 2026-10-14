# Bittopos — Product and Architectural Vision

**Status:** Active strategic vision; implementation is incremental  
**Last reviewed:** 2026-10-14  
**Authority:** Defines product intent and principles. Normative contracts, ADRs, code, and tests govern concrete requirements and verified behavior.

## 1. Purpose

Bittopos is a semantic structuring and execution-control system. It lets people define, organize, navigate, and progressively operationalize explicit models of ideas and work, starting locally and extending to additional interfaces and collaboration when justified.

Its central premise is that a project's structure should be explicit, persistent, inspectable, and controlled by its users. Instead of leaving structure fragmented across conversations, documents, task lists, and an AI model's transient context, Bittopos provides a durable model that can be revised, reused, navigated, and eventually shared.

Bittopos is a general-purpose structural foundation, not a replacement for every editor, task manager, IDE, or specialist tool. Its purpose is to make the relationships and organization of work explicit so that domain-specific workflows can build on them.

## 2. Core thesis

> The authoritative structure of a project should live in a durable, inspectable model controlled by people—not in an AI system's transient context or disconnected documents.

This supports:
- externalizing ideas into a structure people can inspect and revise;
- separating reusable definitions from their concrete occurrences;
- validating and navigating identity, containment, and supported relationships;
- enabling AI to work against persistent state rather than relying on conversational memory;
- connecting future operational views such as tasks, responsibilities, and schedules to the same model;
- reusing the model across interfaces without making one interface authoritative.

The model is constructed by its users; Bittopos does not assume every idea has one objectively correct structure.

## 3. Design principles

1. **Human ownership:** changes proposed by AI or automation must be inspectable and subject to user visibility and control.
2. **Definition and instance are distinct:** a Resource is a reusable definition; an Instance is a concrete occurrence in a workspace.
3. **Local-first:** core workflows remain useful offline; cloud and collaboration are optional extensions.
4. **Domain independence:** the core must not hard-code one industry's vocabulary or workflow.
5. **Explicit navigability:** identity, containment, and supported relationships must be inspectable and traversable.
6. **Progressive complexity:** simple entry points should coexist with richer models without avoidable complexity.
7. **Controlled AI collaboration:** AI can propose and perform authorized operations, but is not the owner or source of truth of the model.
8. **Clear boundaries:** domain rules, application operations, interfaces, and persistence adapters have distinct responsibilities.

## 4. Conceptual model

These terms express intended responsibilities; the current implementation may support only a subset.

- **Resource:** reusable definition with a stable identity.
- **Instance:** identity-bearing occurrence of a Resource within a workspace; instances may be nested by containment.
- **Segment:** named unit grouping related information or capabilities.
- **Detail / Concept:** standard descriptive or key-value information, as defined by the applicable contract.
- **Component:** structured or specialized information that does not fit the standard descriptive role.
- **Mask:** local, segment-scoped differences from a Resource definition. The current conceptual direction is sparse overrides with dynamic inheritance: unmodified values resolve from the latest definition, while locally modified values remain local. The normative contract must define identity, deletion, reset, validation, and conflict edge cases before implementation.

Structural registration belongs to `management.structural`; allocation-related information belongs to `semantic.allocation`. Their precise schemas and invariants are defined by the current model contract, not by this vision.

Future higher-order spaces may include tasks, responsibilities, timelines, taxonomies, and collaboration. They should be compositions or extensions of the shared model, not unjustified parallel data systems.

## 5. AI integration

A typical AI-assisted workflow may interpret intent, propose a structure, explain assumptions and inconsistencies, present changes for review or apply them when authorized, and then read persisted state before continuing later. The goal is to make structural continuity durable and inspectable rather than dependent on whether an agent remembers a conversation.

AI-generated structures may be incomplete or wrong. Generation is not proof of correctness; changes must remain reviewable.

## 6. Interfaces and users

Bittopos is intended for individuals and teams across creative work, research, planning, organizations, and software development—not only technical professionals.

- **CLI:** precise and scriptable operations.
- **Web:** approachable visual editing and navigation.
- **Mobile and desktop:** context-appropriate interaction.
- **AI adapters:** authorized inspection and operations through explicit application boundaries.

The CLI is the operational interface today. The static web prototype is experimental and is not yet connected to the Rust core. Future interfaces must share model semantics even when their presentation differs.

## 7. Scope and non-goals

Strategic scope includes Resource and Instance handling, explicit structure, segments and components, inheritance and local overrides, durable local workspaces, future operational spaces, AI-assisted model work, and optional remote collaboration.

Bittopos is not intended to become a universal document editor, replace specialist tools, require cloud access for basic use, permit unrestricted autonomous mutation without user oversight, or make social feeds the primary representation of project structure.

## 8. Architectural direction

```text
Interfaces: CLI · Web · Mobile · Desktop · AI adapters
                         |
                         v
Application: reusable use cases and orchestration
                         |
                         v
Domain: model, invariants, semantics, resolution
                         ^
                         |
Persistence adapters: serialization, files, journal, recovery
```

The diagram expresses responsibility and dependency direction, not a mandatory folder or crate layout. The domain should not depend on interface presentation, filesystem paths, or a particular serialization format. Application operations should be reusable across interfaces. Persistence adapters must preserve consistency and recovery guarantees without becoming the primary home of semantic rules.

Atomic writes to individual files do not provide atomicity across multiple files. Multi-file operations require explicit guarantees and tests for interruption, recovery, and reconciliation. A generic transaction framework is unnecessary unless real requirements justify it.

## 9. Governance and evolution

- Normative contracts define required data shapes, invariants, and operation semantics.
- ADRs record important decisions, alternatives, and rationale.
- Tests verify behavior the implementation claims to support.
- The README explains current capabilities and points to relevant documentation.
- This vision explains strategic intent and design principles.

When documents conflict, identify and resolve the conflict explicitly. The vision does not silently override a contract, and implementation does not silently redefine either. Implementation may lag the vision; documentation must distinguish agreed, implemented, and verified capabilities.

## 10. Success criteria

Bittopos progresses toward its vision when users can turn ideas into inspectable and navigable structures, state persists independently of conversation, definitions and instances behave predictably, local overrides resolve consistently, interfaces share domain logic, higher-order capabilities build on the common model, and local workflows remain useful without cloud services.

These are strategic outcomes, not claims that the current prototype already satisfies them.
