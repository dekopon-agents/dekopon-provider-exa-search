#!/usr/bin/env python3
"""Offline syntax, documented-schema fixtures, coverage references, and local links."""
import json
import re
from collections import Counter
from pathlib import Path
from urllib.parse import unquote

from jsonschema import Draft202012Validator, FormatChecker
from referencing import Registry, Resource

ROOT = Path(__file__).resolve().parents[1]
IGNORED = {".git", ".venv", ".research", "__pycache__"}


def public_files(suffix):
    return sorted(p for p in ROOT.rglob("*" + suffix)
                  if not any(part in IGNORED for part in p.relative_to(ROOT).parts))


def pointer(document, fragment):
    for part in fragment.removeprefix("/").split("/") if fragment else []:
        key = part.replace("~1", "/").replace("~0", "~")
        document = document[int(key)] if isinstance(document, list) else document[key]
    return document


def heading_ids(text):
    seen = Counter()
    ids = set()
    for title in re.findall(r"^#{1,6}\s+(.+?)\s*#*\s*$", text, re.M):
        slug = re.sub(r"[^\w\- ]", "", title.lower()).replace(" ", "-")
        occurrence = seen[slug]
        seen[slug] += 1
        ids.add(slug + (f"-{occurrence}" if occurrence else ""))
    return ids


def main():
    documents = {p: json.loads(p.read_text()) for p in public_files(".json")}
    schemas = {p: d for p, d in documents.items() if "schemas" in p.relative_to(ROOT).parts}
    registry = Registry().with_resources(
        (p.as_uri(), Resource.from_contents(d)) for p, d in schemas.items()
    )
    for path, schema in schemas.items():
        Draft202012Validator.check_schema(schema)
        # Assert all schema references resolve locally; never use the network.
        def check_refs(node):
            if isinstance(node, dict):
                if "$ref" in node:
                    target, _, fragment = node["$ref"].partition("#")
                    assert not re.match(r"[a-zA-Z]+:", target), f"remote ref: {path}"
                    resolved = (path.parent / target).resolve() if target else path
                    assert resolved in schemas, f"missing schema: {resolved.name}"
                    pointer(schemas[resolved], fragment)
                for value in node.values():
                    check_refs(value)
            elif isinstance(node, list):
                for value in node:
                    check_refs(value)
        check_refs(schema)

    fixtures = documents[ROOT / "examples/fixtures.json"]
    covered = set()
    valid = invalid = 0
    for fixture in fixtures:
        path = ROOT / fixture["file"]
        schema_path = ROOT / fixture["schema"]
        validator = Draft202012Validator(
            {"$ref": schema_path.as_uri()}, registry=registry, format_checker=FormatChecker()
        )
        errors = list(validator.iter_errors(documents[path]))
        if fixture["valid"]:
            assert not errors, f"{fixture['file']}: " + "; ".join(e.message for e in errors[:3])
            valid += 1
        else:
            assert errors, f"invalid fixture unexpectedly accepted: {fixture['file']}"
            invalid += 1
        covered.add(path)
    assert set((ROOT / "examples").rglob("*.json")) == covered | {ROOT / "examples/fixtures.json"}, "unregistered fixture"

    fenced_count = link_count = 0
    for path in public_files(".md"):
        text = path.read_text()
        for fence in re.finditer(r"^```json\s*\n(.*?)^```\s*$", text, re.M | re.S):
            json.loads(fence[1])
            fenced_count += 1
        for link in re.findall(r"\[[^\]]*\]\(([^)\s]+)(?:\s+\"[^\"]*\")?\)", text):
            if re.match(r"[a-zA-Z][a-zA-Z0-9+.-]*:", link):
                continue
            file, _, anchor = unquote(link).partition("#")
            target = (path.parent / file).resolve() if file else path
            assert target.is_relative_to(ROOT), f"link outside repository: {path.name}: {link}"
            assert target.exists(), f"broken link: {path.name}: {link}"
            if anchor and target.suffix == ".md":
                assert anchor in heading_ids(target.read_text()), f"broken anchor: {path.name}: {link}"
            link_count += 1

    inventory = documents[ROOT / "docs/coverage.json"]
    allowed = {"supported", "beta", "deprecated/ignored", "operator-only", "explicitly deferred"}
    identities = set()
    counts = Counter()
    for entry in inventory["parameters"]:
        identity = (entry["endpoint"], entry["location"], entry["path"])
        assert identity not in identities, f"duplicate coverage: {identity}"
        identities.add(identity)
        assert entry["providerStatus"] in allowed and entry["behavior"]
        assert (ROOT / entry["docs"]).is_file()
        for ref in entry["schemaPointers"]:
            pointer(documents[ROOT / entry["schema"]], ref)
        counts[entry["endpoint"]] += 1
    shapes = documents[ROOT / "schemas/upstream/documented-shapes.json"]
    def named_paths(node, prefix="", seen=()):
        if "$ref" in node:
            ref = node["$ref"]
            if ref not in seen:
                yield from named_paths(pointer(shapes, ref.removeprefix("#")), prefix, seen + (ref,))
            return
        for name, child in node.get("properties", {}).items():
            path = f"{prefix}.{name}" if prefix else name
            yield path
            yield from named_paths(child, path, seen)
        if "items" in node:
            yield from named_paths(node["items"], prefix + "[]", seen)
        for combinator in ("anyOf", "oneOf", "allOf"):
            for child in node.get(combinator, []):
                yield from named_paths(child, prefix, seen)
    for entry in inventory["endpoints"]:
        assert (ROOT / entry["docs"]).is_file()
        endpoint = entry["id"]
        expected = set(named_paths(shapes["$defs"][endpoint.title() + "Request"]))
        actual = {path for op, location, path in identities if op == endpoint and location == "body"}
        guide_only = {"useAutoprompt"} if endpoint == "search" else set()
        assert actual == expected | guide_only, f"incomplete named-property coverage: {endpoint}: {expected ^ actual}"
    print(f"PASS: {len(documents)} JSON files; {len(schemas)} well-formed schemas; {fenced_count} JSON fences")
    print(f"PASS: {valid} valid fixtures; {invalid} invalid/conflicting fixtures rejected; {link_count} local links")
    print("PASS: coverage references and unique parameter paths: " + ", ".join(f"{k}={v}" for k, v in sorted(counts.items())))
    print("LIMIT: semantic, operator, native HTTP and live-vendor behavior are documentation-only; see docs/validation.md")


if __name__ == "__main__":
    main()
