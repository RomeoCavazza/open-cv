local M = {}

local function normal(cmd)
  vim.cmd(("silent! normal! %s"):format(cmd))
end

local function get_visual_range()
  local a = vim.fn.getpos("'<")
  local b = vim.fn.getpos("'>")
  if type(a) ~= "table" or type(b) ~= "table" then return nil end
  if (a[2] or 0) == 0 or (b[2] or 0) == 0 then return nil end
  return a, b
end

local function get_text_between_marks()
  local a, b = get_visual_range()
  if not a then return nil end

  local bufnr = vim.api.nvim_get_current_buf()
  local l1, c1 = a[2], a[3]
  local l2, c2 = b[2], b[3]
  if l1 > l2 or (l1 == l2 and c1 > c2) then
    l1, l2 = l2, l1
    c1, c2 = c2, c1
  end

  local lines = vim.api.nvim_buf_get_lines(bufnr, l1 - 1, l2, false)
  if #lines == 0 then return nil end

  c1 = math.max(c1, 1)
  c2 = math.max(c2, 1)

  if #lines == 1 then
    lines[1] = string.sub(lines[1], c1, c2)
  else
    lines[1] = string.sub(lines[1], c1)
    lines[#lines] = string.sub(lines[#lines], 1, c2)
  end

  return table.concat(lines, "\n")
end

local function set_plus(text)
  if not text or text == "" then return false end
  vim.fn.setreg("+", text)
  return true
end

function M.copy()
  local marked = get_text_between_marks()
  if marked and marked ~= "" then
    set_plus(marked)
    return
  end

  local m = vim.api.nvim_get_mode().mode
  if m == "v" or m == "V" or m == "\22" then
    normal([["+y]])
  else
    normal([["+yy]])
  end
end

function M.cut()
  local m = vim.api.nvim_get_mode().mode
  if m == "v" or m == "V" or m == "\22" then
    normal([["+d]])
  else
    normal([["+dd]])
  end
end

function M.paste()
  normal([["+p]])
end

function M.select_all()
  normal([[ggVG]])
end

function M.inspect()
  if vim.fn.exists(":Inspect") == 2 then
    vim.cmd("Inspect")
    return
  end
  if vim.lsp and vim.lsp.buf and vim.lsp.buf.hover then
    vim.lsp.buf.hover()
  end
end

local function kill_menupopup_autocmds()
  local ok, list = pcall(vim.api.nvim_get_autocmds, { event = "MenuPopup" })
  if not ok then return end
  for _, ac in ipairs(list) do
    if ac.id then pcall(vim.api.nvim_del_autocmd, ac.id) end
  end
end

function M.setup_menu()
  pcall(vim.cmd, "silent! aunmenu PopUp")
  pcall(vim.cmd, "silent! aunmenu PopUp.*")

  vim.cmd([[amenu <silent> PopUp.Copy\ (Ctrl+C)          :lua require("core.popup").copy()<CR>]])
  vim.cmd([[amenu <silent> PopUp.Paste\ (Ctrl+V)         :lua require("core.popup").paste()<CR>]])
  vim.cmd([[amenu <silent> PopUp.Cut\ (Ctrl+X)           :lua require("core.popup").cut()<CR>]])
  vim.cmd([[amenu <silent> PopUp.Select\ All\ (Ctrl+A)   :lua require("core.popup").select_all()<CR>]])
  vim.cmd([[amenu <silent> PopUp.Inspect                :lua require("core.popup").inspect()<CR>]])

  kill_menupopup_autocmds()
end

return M
