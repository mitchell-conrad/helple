defmodule WordleCompanionWeb.Counter do
  use WordleCompanionWeb, :live_view

  def mount(_params, _session, socket) do
    {:ok,
     assign(socket,
       throttle_rate: 75,
       solution: "",
       guess_1: "",
       guess_2: "",
       guess_3: "",
       guess_4: "",
       guess_5: "",
       guess_6: "",
       remaining_1: 0,
       remaining_2: 0,
       remaining_3: 0,
       remaining_4: 0,
       remaining_5: 0,
       remaining_6: 0
     )}
  end

  def handle_event("eval", session, socket) do
    {time, nums} =
      :timer.tc(&WordleCompanion.RustSolver.external_calc/2, [
        session["solution_field"],
        [
          session["guess_1_field"],
          session["guess_2_field"],
          session["guess_3_field"],
          session["guess_4_field"],
          session["guess_5_field"],
          session["guess_6_field"]
        ]
      ])

    IO.inspect(%{time: time / 1000, vals: nums})

    {:noreply,
     assign(socket, %{
       solution: session["solution_field"],
       guess_1: session["guess_1_field"],
       guess_2: session["guess_2_field"],
       guess_3: session["guess_3_field"],
       guess_4: session["guess_4_field"],
       guess_5: session["guess_5_field"],
       guess_6: session["guess_6_field"],
       remaining_1: Enum.at(nums, 0),
       remaining_2: Enum.at(nums, 1),
       remaining_3: Enum.at(nums, 2),
       remaining_4: Enum.at(nums, 3),
       remaining_5: Enum.at(nums, 4),
       remaining_6: Enum.at(nums, 5)
     })}
  end

  def render(assigns) do
    ~H"""

    <form phx-change="eval" phx-submit="eval">
    <input
      type="text"
      name="solution_field"
      value={@solution}
      placeholder="Solution"
      maxlength="5"
    />
    <div>
      <input
        type="text"
        name="guess_1_field"
        value={@guess_1}
        maxlength="5"
      />
      <h2><%= @remaining_1 %></h2>
    </div>
    <div>
      <input
        type="text"
        name="guess_2_field"
        value={@guess_2}
        maxlength="5"
      />
      <h2><%= @remaining_2 %></h2>
    </div>
    <div>
      <input
        type="text"
        name="guess_3_field"
        value={@guess_3}
        maxlength="5"
      />
      <h2><%= @remaining_3 %></h2>
    </div>
    <div>
      <input
        type="text"
        name="guess_4_field"
        value={@guess_4}
        maxlength="5"
      />
      <h2><%= @remaining_4 %></h2>
    </div>
    <div>
      <input
        type="text"
        name="guess_5_field"
        value={@guess_5}
        maxlength="5"
      />
      <h2><%= @remaining_5 %></h2>
    </div>
    <div>
      <input
        type="text"
        name="guess_6_field"
        value={@guess_6}
        maxlength="5"
      />
      <h2><%= @remaining_6 %></h2>
    </div>
    </form>

    """
  end
end
