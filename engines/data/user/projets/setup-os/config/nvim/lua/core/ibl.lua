local M = {}

function M.setup()
  local ok_hooks, hooks = pcall(require, "ibl.hooks")

  local function define_hl()
    pcall(vim.api.nvim_set_hl, 0, "IblIndent", { fg = "#444444", nocombine = true })
    pcall(vim.api.nvim_set_hl, 0, "IblScope",  { fg = "#888888", nocombine = true })
  end

  if ok_hooks then
    hooks.register(hooks.type.HIGHLIGHT_SETUP, define_hl)
  end

  define_hl()

  require("ibl").setup({
    indent = { highlight = "IblIndent" },
    scope  = { enabled = true, highlight = "IblScope" },
  })
end

return M
