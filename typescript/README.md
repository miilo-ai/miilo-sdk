<h1 align="center">Miilo Adapter SDK for TypeScript</h1>

<p align="center">
  <img alt="TypeScript" src="https://img.shields.io/badge/TypeScript-3178c6?style=flat-square&amp;logo=typescript&amp;logoColor=white" />
</p>

Strict TypeScript declarations for Miilo Protocol v0.1 and provider adapters.

The package is licensed under Apache-2.0. Package publication follows the SDK
release process and its validation gates.

```typescript
import type { MiiloAdapter, ObservatorySnapshot } from "@miilo/adapter-sdk";
```

Adapters retain provider authority. An advertised action still requires provider
authorization, the declared confirmation, and an exact expected revision.
