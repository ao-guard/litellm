import re
from pathlib import Path

LITELLM = Path(__file__).resolve().parents[3] / "litellm"
ALLOWED = (LITELLM / "repositories", LITELLM / "rust_bridge")
RUST_REPOSITORY_USE = re.compile(r"\bRust\w+Repository\b|\brust_database\b")


def test_only_python_repositories_reach_rust_repositories() -> None:
    offenders = [
        f"{path.relative_to(LITELLM.parent)}:{number}: {line.strip()}"
        for path in sorted(LITELLM.rglob("*.py"))
        if not any(path.is_relative_to(allowed) for allowed in ALLOWED)
        for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1)
        if RUST_REPOSITORY_USE.search(line)
    ]

    assert offenders == [], "use a Python repository in litellm/repositories/ instead:\n" + "\n".join(offenders)
