# `@miilo/adapter-sdk`

Strict TypeScript declarations for Miilo Protocol v0.1 and provider adapters.

This package is private during incubation. The intended publication boundary is
Apache-2.0 after legal and product review.

```typescript
import type { MiiloAdapter, ObservatorySnapshot } from "@miilo/adapter-sdk";
```

Adapters retain provider authority. An advertised action still requires provider
authorization, the declared confirmation, and an exact expected revision.
