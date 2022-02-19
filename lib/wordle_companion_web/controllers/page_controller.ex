defmodule WordleCompanionWeb.PageController do
  use WordleCompanionWeb, :controller

  def index(conn, _params) do
    render(conn, "index.html")
  end
end
