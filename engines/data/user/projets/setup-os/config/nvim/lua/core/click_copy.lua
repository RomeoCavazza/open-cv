local M = {}

local function set_plus(text)
  if not text or text == "" then return false end
  vim.fn.setreg("+", text)
  return true
end

local function buf_text(buf)
  local lines = vim.api.nvim_buf_get_lines(buf, 0, -1, false)
  return table.concat(lines, "\n")
end

local function notify_once(msg)
  vim.notify(msg, vim.log.levels.INFO, { title = "Clipboard" })
end

function M.attach(buf)
  if not buf or not vim.api.nvim_buf_is_valid(buf) then return end

  if vim.b[buf].sg_click_copy_attached then return end
  vim.b[buf].sg_click_copy_attached = true

  local state = { timer = nil }

  local function cancel_timer()
    if state.timer then
      state.timer:stop()
      state.timer:close()
      state.timer = nil
    end
  end

  local function single_click_action()
    local line = vim.api.nvim_get_current_line()
    if not set_plus(line) then
      set_plus(buf_text(buf))
    end
    notify_once("Copied")
  end

  local function double_click_action()
    set_plus(buf_text(buf))
    notify_once("Copied")
  end

  vim.keymap.set("n", "<LeftMouse>", function()
    pcall(vim.api.nvim_input, "<LeftMouse>")

    cancel_timer()
    state.timer = vim.loop.new_timer()
    state.timer:start(220, 0, vim.schedule_wrap(function()
      state.timer:stop()
      state.timer:close()
      state.timer = nil
      single_click_action()
    end))
  end, { buffer = buf, silent = true, nowait = true })

  vim.keymap.set("n", "<2-LeftMouse>", function()
    pcall(vim.api.nvim_input, "<2-LeftMouse>")
    cancel_timer()
    double_click_action()
  end, { buffer = buf, silent = true, nowait = true })

  vim.keymap.set("n", "y", function()
    cancel_timer()
    set_plus(buf_text(buf))
    notify_once("Copied")
  end, { buffer = buf, silent = true, nowait = true })
end

return M
