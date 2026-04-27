local M = {}

local BLUE = "#00F0FF"

local function set_hl()
  pcall(vim.api.nvim_set_hl, 0, "NormalFloat", { bg = "NONE" })
  pcall(vim.api.nvim_set_hl, 0, "FloatBorder", { fg = BLUE, bg = "NONE" })
  pcall(vim.api.nvim_set_hl, 0, "WinSeparator", { fg = BLUE, bg = "NONE" })
  pcall(vim.api.nvim_set_hl, 0, "VertSplit", { fg = BLUE, bg = "NONE" })

  pcall(vim.api.nvim_set_hl, 0, "LazyBorder", { fg = BLUE, bg = "NONE" })
end

local function is_float(win)
  local ok, cfg = pcall(vim.api.nvim_win_get_config, win)
  return ok and cfg and cfg.relative and cfg.relative ~= ""
end

function M.apply_to(win)
  if type(win) ~= "number" then return end
  if not vim.api.nvim_win_is_valid(win) then return end
  if not is_float(win) then return end

  set_hl()

  pcall(vim.api.nvim_win_set_option, win, "winhighlight", "Normal:NormalFloat,FloatBorder:FloatBorder")
  pcall(vim.api.nvim_win_set_option, win, "winblend", 0)
end

function M.setup()
  set_hl()

  vim.api.nvim_create_autocmd({ "WinNew", "WinEnter" }, {
    callback = function(args)
      local win = (args and type(args.win) == "number") and args.win or vim.api.nvim_get_current_win()
      M.apply_to(win)
    end,
  })

  vim.api.nvim_create_autocmd("ColorScheme", {
    callback = function()
      set_hl()
      vim.defer_fn(function() M.apply_to(vim.api.nvim_get_current_win()) end, 10)
      vim.defer_fn(function() M.apply_to(vim.api.nvim_get_current_win()) end, 50)
    end,
  })

  vim.api.nvim_create_autocmd("FileType", {
    pattern = "lazy",
    callback = function()
      set_hl()
      vim.opt_local.winhighlight = "Normal:NormalFloat,FloatBorder:LazyBorder"
      vim.opt_local.winblend = 0
    end,
  })
end

return M
