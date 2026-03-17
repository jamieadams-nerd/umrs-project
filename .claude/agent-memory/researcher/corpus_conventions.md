---
name: corpus_conventions
description: Conventions for .claude/references/ RAG collections — naming, SOURCE.md format, tracking
type: reference
---

## Directory Structure

.claude/references/<collection-name>/
  SOURCE.md         — provenance: URLs, retrieval date, description
  <content files>

## Standalone Reference Files

Some references are single files placed directly under .claude/references/:
  .claude/references/xdg-basedir-spec.md   — XDG Base Directory Spec (from training knowledge)
  .claude/references/capability-matrix-domains.md

## Provenance Requirements (lighter than refs/)

- No SHA-256 required (unlike refs/ which requires checksums)
- SOURCE.md or header block with source URL and retrieval date is sufficient
- Mark training-knowledge documents with "Status: requires_verification_download"
  and include the source URL for Jamie to verify manually

## refs/ vs .claude/references/

- refs/          — official security standards; strict provenance; SHA-256 required; manifest tracked
- .claude/references/ — RAG augmentation; lighter process; broader technical content

## RAG Collections Tracking

Active collection URLs are tracked in rag-collections.md (auto-memory).
Check that file before creating a new collection to avoid duplication.
