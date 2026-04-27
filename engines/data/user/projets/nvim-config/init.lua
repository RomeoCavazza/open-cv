require("core.bootstrap")
require("core.options")
require("core.keymaps")
require("core.autocmds")

-- Fallback theme so highlights never stay "cleared" if plugins fail
pcall(vim.cmd, "colorscheme default")

require("lazy").setup("plugins", {
  change_detection = { notify = false },
  ui = { border = "rounded" },
})
