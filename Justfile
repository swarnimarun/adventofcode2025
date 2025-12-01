# Project directory prefix
project_prefix := "aoc_"

# Cross-platform shell configuration
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# Run a project by language
# Usage:
#   just run rust           # runs cargo run in aoc_rust
#   just run rust -- arg1 arg2 "arg with space"
#   just run zig -- --flag 123
run language *args:
    {{ if language == "rust" { "just _rust " + args } else if language == "zig" { "just _zig " + args } else if language == "haskell" { "just _haskell " + args } else if language == "fsharp" { "just _fsharp " + args } else if language == "elixir" { "just _elixir " + args } else if language == "kotlin" { "just _kotlin " + args } else if language == "deno" { "just _deno " + args } else if language == "swift" { "just _swift " + args } else if language == "uiua" { "just _uiua " + args } else { "just _unknown " + language } }}

# Language-specific runners (private recipes)
_rust *args:
    cd {{project_prefix}}rust && cargo run -- {{args}}

_uiua *args:
    cd {{project_prefix}}uiua && just run {{args}}

_zig *args:
    cd {{project_prefix}}zig && zig build run -- {{args}}

_haskell *args:
    cd {{project_prefix}}haskell && stack run -- {{args}}

_fsharp *args:
    cd {{project_prefix}}fsharp && dotnet run -- {{args}}

_elixir *args:
    cd {{project_prefix}}elixir && mix run --no-halt -- {{args}}

_kotlin *args:
    #!/usr/bin/env bash
    set -euo pipefail
    cd {{project_prefix}}kotlin
    if [ "{{os_family()}}" = "windows" ]; then
        ./gradlew.bat run --args="{{args}}"
    else
        ./gradlew run --args="{{args}}"
    fi

_deno *args:
    cd {{project_prefix}}deno && deno run --allow-all main.ts {{args}}

_swift *args:
    cd {{project_prefix}}swift && swift run {{args}}

_unknown language:
    @echo "Error: Unknown language '{{language}}'" >&2
    @echo "Supported: rust, zig, haskell, fsharp, elixir, kotlin, deno, swift" >&2
    @exit 1
