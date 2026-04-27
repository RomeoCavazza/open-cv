local M = {}

local ping_cache = "…"
local timer_started = false

local function update_ping()
  local target = "1.1.1.1"
  local cmd = string.format(
    "ping -c 1 -W 1 %s 2>/dev/null | sed -n 's/.*time=\\([0-9.]*\\) ms.*/\\1/p' | head -n1",
    target
  )
  local out = vim.fn.system(cmd)
  out = (out or ""):gsub("%s+$", "")
  if out ~= "" then
    ping_cache = out .. "ms"
  else
    ping_cache = "down"
  end
end

function M.start()
  if timer_started then return end
  timer_started = true
  update_ping()
  local t = vim.loop.new_timer()
  t:start(20000, 20000, vim.schedule_wrap(update_ping))
end

function M.file()
  local name = vim.fn.expand("%:t")
  if name == "" then name = "[No Name]" end
  return name
end

function M.modified()
  return vim.bo.modified and "[+]" or "[-]"
end

function M.penguin()
  return " "
end

function M.ping()
  return ping_cache == "down" and " down" or (" " .. ping_cache)
end

function M.percent()
  local total = math.max(vim.fn.line("$"), 1)
  local pct = math.floor((vim.fn.line(".") * 100) / total)
  return tostring(pct) .. "%%"
end

function M.time()
  return os.date("%H:%M")
end

return M
