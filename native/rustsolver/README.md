# NIF for Elixir.RustSolver

## To build the NIF module:

- Your NIF will now build along with your project.

## To load the NIF:

```elixir
defmodule RustSolver do
    use Rustler, otp_app: :wordle_companion, crate: "rustsolver"

    # When your NIF is loaded, it will override this function.
    def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end
```
