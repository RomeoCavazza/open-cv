local theme = require("core.theme")
local force_floats = require("core.force_floats")

local function apply_all()
  theme.apply()
end

local function apply_late()
  apply_all()
  vim.defer_fn(apply_all, 50)
  vim.defer_fn(apply_all, 200)
  vim.defer_fn(apply_all, 800)
end

force_floats.setup()

vim.api.nvim_create_autocmd({ "VimEnter", "UIEnter", "ColorScheme" }, {
  callback = apply_late,
})

vim.api.nvim_create_autocmd("User", {
  pattern = "VeryLazy",
  callback = apply_late,
})

apply_late()

vim.api.nvim_create_autocmd("TextYankPost", {
  callback = function()
    vim.highlight.on_yank()
  end,
})

vim.api.nvim_create_autocmd("FileType", {
  pattern = { "notify", "noice" },
  callback = function(ev)
    local ok, cc = pcall(require, "core.click_copy")
    if ok then
      cc.attach(ev.buf)
    end
  end,
})
