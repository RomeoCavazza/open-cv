local function resolve_shellgeist_dir()
  local candidates = {
    "~/Bureau/projets/shellgeist/nvim",
    "~/Bureau/projets/benchmark/shellgeist/nvim",
  }

  for _, candidate in ipairs(candidates) do
    local expanded = vim.fn.expand(candidate)
    if vim.fn.isdirectory(expanded) == 1 then
      return expanded
    end
  end

  return nil
end

local plugin_dir = resolve_shellgeist_dir()
if not plugin_dir then
  vim.schedule(function()
    vim.notify(
      "ShellGeist plugin directory not found. Checked ~/Bureau/projets/shellgeist/nvim and ~/Bureau/projets/benchmark/shellgeist/nvim",
      vim.log.levels.WARN,
      { title = "ShellGeist" }
    )
  end)
  return {}
end

return {
  {
    dir = plugin_dir,
    name = "shellgeist",
    config = function()
      local ok, user = pcall(require, "user.shellgeist")
      require("shellgeist").setup(ok and user or {
        socket = vim.fn.expand("~/.cache/shellgeist.sock"),
      })

      vim.keymap.set("n", "<leader>ag", function()
        require("shellgeist").set_mode("auto")
        vim.cmd("SGAgent")
      end, { desc = "ShellGeist: Auto Mode (sidebar + prompt)" })
      vim.keymap.set("n", "<leader>ar", function()
        require("shellgeist").set_mode("review")
        vim.cmd("SGAgent")
      end, { desc = "ShellGeist: Review Mode (sidebar + prompt)" })
    end,
  },
}
