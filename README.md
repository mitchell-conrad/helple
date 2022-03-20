# Helple

## Development
Deps:
* A nightly rust compiler
* Elixir

### Local dev server
* Install dependencies with `mix deps.get`
* Start Phoenix endpoint with `mix phx.server` or inside IEx with `iex -S mix phx.server`
* Visit locally served site at [`localhost:4000`](http://localhost:4000)

### Docker dev server
* Enable IPv6 support for docker
* Install dependencies with `mix deps.get`

```
# Install dependencies (once off)
mix deps.get
# Generate a secret key (once off)
export SECRET_KEY_BASE=`MIX_DEBUG=0 mix phx.gen.secret`
# Build the docker image
docker build . -t helple-docker
# Run the docker image, exposing server on localhost port 4000
docker run  -e SECRET_KEY_BASE=$SECRET_KEY_BASE -e PHX_HOST=localhost -e MIX_ENV=prod -p 4000:4000 helple-docker
```

## Deployment
`fly deploy`




