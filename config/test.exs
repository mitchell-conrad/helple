import Config

# We don't run a server during test. If one is required,
# you can enable the server option below.
config :wordle_companion, WordleCompanionWeb.Endpoint,
  http: [ip: {127, 0, 0, 1}, port: 4002],
  secret_key_base: "URccuAiHm1O08MkjeV74p9Jtm4UTl/j4MLhgfWe27rvLYh1zLI/FR1oSTozEWGSY",
  server: false

# Print only warnings and errors during test
config :logger, level: :warn

# Initialize plugs at runtime for faster test compilation
config :phoenix, :plug_init_mode, :runtime
