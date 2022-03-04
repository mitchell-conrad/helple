# NIF for Elixir.RustSolver

## Dependencies
* Nightly rust compiler

## Development
### Please lint and format before submitting a CR
```bash
cargo clippy
cargo fmt
```

### Build
```bash
cargo build
```

### Test
```bash
cargo test
```

### Benchmark
```bash
cargo bench
# Document any performance regressions in the CR
```

## To load the NIF:

```elixir
defmodule RustSolver do
    use Rustler, otp_app: :wordle_companion, crate: "rustsolver"

    # When your NIF is loaded, it will override this function.
    def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end
```
