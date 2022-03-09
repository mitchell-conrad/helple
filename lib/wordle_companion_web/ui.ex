defmodule WordleCompanionWeb.UI do
  use Phoenix.Component

  def nav(assigns) do
    ~H"""
      <div class="pt-6 flex space-x-2">
        <.nav_button active={@active} name="remaining" body="Remaining Words" handler="mode-remaining">
        </.nav_button>
        <.nav_button active={@active} name="stats" body="Stats" handler="mode-stats">
        </.nav_button>
      </div>
    """
  end

  def nav_button(assigns) do
    ~H"""
      <%= if @active == @name do %>
        <button
          type="button"
          class="p-1 flex-1 cursor-not-allowed h-20 text-4xl font-bold bg-indigo-500 rounded-md text-white text-center outline outline-lime-500 outline-4"
          phx-click={@handler} >
          <%= @body %>
        </button>
      <% else %>
        <button
          type="button"
          class="p-1 flex-1 h-20 text-4xl font-bold bg-indigo-600 rounded-md text-white text-center"
          phx-click={@handler} >
          <%= @body %>
        </button>
      <% end %>
    """
  end
end
