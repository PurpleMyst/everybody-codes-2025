# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "typer",
#     "python-dotenv",
#     "requests",
#     "termcolor",
#     "tomlkit",
#     "tabulate",
# ]
# ///
import shlex
import subprocess
import sys
import typing as t
from contextlib import chdir
from datetime import datetime
from functools import partial, wraps
from os import environ
from pathlib import Path

import requests
import tomlkit as toml
import typer
from dotenv import load_dotenv
from termcolor import colored as c

app = typer.Typer(context_settings={"help_option_names": ["-h", "--help"]})
cb = partial(c, attrs=["bold"])

PROBLEM_NAME = "quest"

MAIN = """\
fn main() {{
    let (part1, part2, part3) = {crate}::solve();
    println!("{{part1}}");
    println!("{{part2}}");
    println!("{{part3}}");
}}\
"""

LIB = """\
use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    "TODO"
}

#[inline]
pub fn solve_part2() -> impl Display {
    "TODO"
}

#[inline]
pub fn solve_part3() -> impl Display {
    "TODO"
}\
"""

DEFAULT_BASELINE = "previous"

WORKSPACE_MANIFEST_PATH = Path(__file__).parent / "Cargo.toml"

load_dotenv()

session = requests.Session()
session.headers.update({"User-Agent": "PurpleMyst/aoc-template with much love! <3"})


def run(cmd: t.Sequence[str | Path], /, **kwargs) -> subprocess.CompletedProcess:
    check = kwargs.pop("check", True)
    print(
        cb("$", "green"),
        shlex.join(map(str, cmd)),
        c(f"(w/ options {kwargs})", "green") if kwargs else "",
    )
    proc = subprocess.run(cmd, **kwargs)
    if check and proc.returncode != 0:
        print(cb("Failed.", "red"))
        sys.exit(proc.returncode)
    return proc


def add_line(p: Path, l: str) -> None:
    ls = p.read_text().splitlines()
    ls.insert(-1, l)
    if ls[-1] != "":
        # add or keep trailing newline
        ls.append("")
    p.write_text("\n".join(ls), newline="\n")


def in_root_dir(f):
    @wraps(f)
    def inner(*args, **kwargs):
        with chdir(Path(__file__).parent):
            return f(*args, **kwargs)

    return inner


@app.command()
@in_root_dir
def start_solve(num: int) -> None:
    "Start solving a problem."
    crate = f"{PROBLEM_NAME}{int(num):02}"
    crate_path = Path(crate)

    if crate_path.exists():
        print(f"{crate} already exists.")
        return

    manifest = toml.parse(WORKSPACE_MANIFEST_PATH.read_text())
    if crate not in manifest["workspace"]["members"]:  # type: ignore
        manifest["workspace"]["members"].append(crate)  # type: ignore

    metadata = manifest["workspace"].setdefault("metadata", {})  # type: ignore
    metadata[crate] = {"start_time": datetime.now()}

    with WORKSPACE_MANIFEST_PATH.open("w") as manifest_f:
        toml.dump(manifest, manifest_f)

    run(("cargo", "new", "--bin", crate))
    run(
        (
            "cargo",
            "add",
            "--manifest-path",
            "benchmark/Cargo.toml",
            "--path",
            crate,
            crate,
        )
    )

    src = crate_path / "src"
    (src / "main.rs").write_text(MAIN.format(crate=crate), newline="\n")
    (src / "lib.rs").write_text(LIB, newline="\n")

    benches = Path("benchmark", "benches")
    add_line(benches / "criterion.rs", f"    {crate},")

    run(("git", "add", crate))


app.command("ss")(start_solve)


@app.command()
@in_root_dir
def set_baseline(
    *, pattern: t.Annotated[str, typer.Argument()] = ".", name: str = DEFAULT_BASELINE
) -> None:
    "Run a criterion benchmark, setting its results as the new baseline."
    run(
        (
            "cargo",
            "bench",
            "--bench",
            "criterion",
            "--",
            pattern,
            "--save-baseline",
            name,
            "--verbose",
        )
    )


app.command("sb")(set_baseline)


@app.command()
@in_root_dir
def compare(
    *, pattern: t.Annotated[str, typer.Argument()] = ".", name: str = DEFAULT_BASELINE
) -> None:
    "Run a criterion benchmark, comparing its results to the saved baseline."
    run(
        (
            "cargo",
            "bench",
            "--bench",
            "criterion",
            "--",
            pattern,
            "--baseline",
            name,
            "--verbose",
        )
    )


app.command("cmp")(compare)


@app.command()
@in_root_dir
def compare_by_stashing(
    *, pattern: t.Annotated[str, typer.Argument()] = ".", name: str = DEFAULT_BASELINE
) -> None:
    "Stash the current changes, set the baseline and then compare the new changes."
    run(("git", "stash", "push", "-m", "Stashing for benchmarking"))
    set_baseline(pattern=pattern, name=name)
    run(("git", "stash", "pop"))
    compare(pattern=pattern, name=name)


app.command("cmp-stash")(compare_by_stashing)


@app.command()
@in_root_dir
def measure_completion_time() -> None:
    "Measure completion time for all problems."
    from tabulate import tabulate

    manifest = toml.parse(WORKSPACE_MANIFEST_PATH.read_text())

    table = []
    for problem in Path().glob(f"{PROBLEM_NAME}*"):
        metadata = manifest["workspace"].get("metadata", {}).get(problem.name, {})  # type: ignore
        start_time = metadata.get("start_time")
        end_time = metadata.get("completion_time")
        src = problem / "src"
        if start_time is None:
            start_time = datetime.fromtimestamp((src / "input.txt").stat().st_ctime)
        if end_time is None:
            end_time = datetime.fromtimestamp(max(f.stat().st_mtime for f in src.glob("**/*.rs")))
        completion_time = end_time - start_time
        table.append((problem.name, str(completion_time)))
    print(tabulate(table, headers=[PROBLEM_NAME.title(), "Completion Time"], tablefmt="fancy_grid"))


app.command("mct")(measure_completion_time)


@app.command()
def set_completion_time() -> None:
    "Set the completion time for the problem you're currently in."

    problem = Path.cwd().resolve().name
    if not problem.startswith(PROBLEM_NAME):
        print(cb(f"Not in a {PROBLEM_NAME} directory.", "red"))
        return

    manifest = toml.parse(WORKSPACE_MANIFEST_PATH.read_text())
    metadata = manifest["workspace"].setdefault("metadata", {})  # type: ignore
    problem_metadata = metadata.setdefault(problem, {})
    if "completion_time" in problem_metadata:
        print(cb("Completion time is already set.", "yellow"))
        return
    problem_metadata["completion_time"] = datetime.now()

    with WORKSPACE_MANIFEST_PATH.open("w") as manifest_f:
        toml.dump(manifest, manifest_f)


app.command("sct")(set_completion_time)


def main() -> None:
    environ["RUSTFLAGS"] = "-C target-cpu=native"
    app()


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("Bye!")
