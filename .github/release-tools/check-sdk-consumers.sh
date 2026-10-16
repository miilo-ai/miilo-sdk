#!/usr/bin/env bash
set -euo pipefail
root=$(pwd)
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
uv build --project python --out-dir "$scratch"
uv venv "$scratch/venv"
uv pip install --python "$scratch/venv/bin/python" "$scratch"/*.whl
(cd "$scratch" && SDK_FIXTURE="$root/protocol/fixtures/factory.snapshot.json" "$scratch/venv/bin/python" -c 'import json, os; from pathlib import Path; from pydantic import TypeAdapter; from miilo_adapter_sdk import ObservatorySnapshot; TypeAdapter(ObservatorySnapshot).validate_python(json.loads(Path(os.environ["SDK_FIXTURE"]).read_text()))')
(cd typescript && npm pack --pack-destination "$scratch" >/dev/null)
npm install --prefix "$scratch/node" "$scratch"/*.tgz typescript@5.9.3
printf 'import type { ObservatorySnapshot } from "@miilo-ai/adapter-sdk";\nconst providers = (snapshot: ObservatorySnapshot) => snapshot.providers;\n' > "$scratch/node/consumer.mts"
(cd "$scratch/node" && ./node_modules/.bin/tsc --noEmit --module nodenext --target ES2022 consumer.mts)
echo 'Installed wheel and npm SDK consumers passed.'
