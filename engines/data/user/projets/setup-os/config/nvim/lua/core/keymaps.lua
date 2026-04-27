vim.g.mapleader = " "
vim.g.maplocalleader = " "

local map = function(mode, lhs, rhs, desc)
  vim.keymap.set(mode, lhs, rhs, { silent = true, desc = desc })
end

local function focus_editor_window()
  local ignore = {
    ["NvimTree"] = true,
    ["Avante"] = true,
    ["AvanteInput"] = true,
    ["trouble"] = true,
    ["Trouble"] = true,
    ["toggleterm"] = true,
    ["alpha"] = true,
    ["lazy"] = true,
    ["mason"] = true,
    ["notify"] = true,
    ["noice"] = true,
  }

  local start = vim.api.nvim_get_current_win()
  for _ = 1, 20 do
    local buf = vim.api.nvim_win_get_buf(vim.api.nvim_get_current_win())
    local ft = vim.bo[buf].filetype
    if not ignore[ft] then
      return true
    end
    pcall(vim.cmd, "wincmd w")
    if vim.api.nvim_get_current_win() == start then
      break
    end
  end
  return false
end

map("n", "<leader>e", "<cmd>NvimTreeToggle<cr>", "Tree toggle")
map("n", "<leader>a", "<cmd>AvanteToggle<cr>", "Avante toggle")
map("n", "<leader>h", "<cmd>SGDashboardToggle<cr>", "Dashboard toggle")

map("n", "<leader>i", "<cmd>CursorMode<cr>", "CursorMode toggle")

map({ "n", "i", "v" }, "<leader>t", function()
  local was_insert = vim.api.nvim_get_mode().mode:match("^i") ~= nil
  if was_insert then vim.cmd("stopinsert") end

  focus_editor_window()
  pcall(vim.cmd, "ToggleTerm")

  if was_insert then vim.cmd("startinsert") end
end, "Terminal toggle (main)")

map("n", "<leader>x", function()
  focus_editor_window()
  pcall(vim.cmd, "Trouble diagnostics toggle")
end, "Trouble diagnostics toggle (main)")

map({ "n", "i", "v" }, "<C-a>", function()
  if vim.api.nvim_get_mode().mode:match("^i") then vim.cmd("stopinsert") end
  vim.cmd([[normal! ggVG]])
end, "Select all")

map("v", "<C-c>", '"+y', "Copy selection (best-effort)")
map("n", "<C-c>", '"+yy', "Copy line (best-effort)")
map({ "n", "v" }, "<C-v>", '"+p', "Paste (best-effort)")
map("i", "<C-v>", '<Esc>"+pa', "Paste (best-effort)")

map("v", "<leader>y", '"+y', "Copy selection (+)")
map("n", "<leader>p", '"+p', "Paste (+)")

local function save()
  if vim.bo.readonly or not vim.bo.modifiable then
    local force = vim.fn.confirm("Buffer is readonly. Force write (!)?", "&Yes\n&No", 2)
    if force == 1 then pcall(vim.cmd, "write!") end
    return
  end
  local ok = pcall(vim.cmd, "write")
  if not ok then
    local force = vim.fn.confirm("Write failed. Force write (!)?", "&Yes\n&No", 2)
    if force == 1 then pcall(vim.cmd, "write!") end
  end
end

local function quit_buffer()
  if vim.bo.modified then
    local choice = vim.fn.confirm("Save changes?", "&Yes\n&No\n&Cancel", 1)
    if choice == 3 then return end
    if choice == 1 then save() end
  end
  pcall(vim.cmd, "bd")
end

map({ "n", "i", "v" }, "<C-s>", function()
  local was_insert = vim.api.nvim_get_mode().mode:match("^i") ~= nil
  if was_insert then vim.cmd("stopinsert") end
  save()
  if was_insert then vim.cmd("startinsert") end
end, "Save")

map({ "n", "i" }, "<C-q>", function()
  local was_insert = vim.api.nvim_get_mode().mode:match("^i") ~= nil
  if was_insert then vim.cmd("stopinsert") end
  quit_buffer()
end, "Quit buffer")

map({ "n", "i" }, "<C-k>", function()
  local was_insert = vim.api.nvim_get_mode().mode:match("^i") ~= nil
  if was_insert then vim.cmd("stopinsert") end
  vim.cmd([[normal! dd]])
end, "Cut line")

pcall(function()
  require("core.popup").setup_menu()
end)
