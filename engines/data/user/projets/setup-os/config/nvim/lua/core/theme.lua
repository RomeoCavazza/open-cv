local M = {}

local BLUE = "#00F0FF"

local function hl(group, spec)
  pcall(vim.api.nvim_set_hl, 0, group, spec)
end

function M.apply()
  for _, g in ipairs({
    "Normal", "NormalNC", "EndOfBuffer", "NonText",
    "SignColumn", "FoldColumn", "LineNr", "CursorLineNr",
    "StatusLine", "StatusLineNC",
    "TabLine", "TabLineFill", "TabLineSel",
    "Pmenu", "PmenuSel", "PmenuSbar", "PmenuThumb",
    "MsgArea", "MsgSeparator",
    "NormalFloat",
  }) do
    hl(g, { bg = "NONE" })
  end

  for _, g in ipairs({
    "FloatBorder", "WinSeparator", "VertSplit",
    "TelescopeBorder", "TelescopePromptBorder", "TelescopeResultsBorder", "TelescopePreviewBorder",
    "NoicePopupBorder", "NoiceCmdlinePopupBorder", "NoicePopupmenuBorder", "NoiceMiniBorder",
    "WhichKeyBorder",
    "NotifyINFOBorder", "NotifyWARNBorder", "NotifyERRORBorder", "NotifyDEBUGBorder", "NotifyTRACEBorder",
    "NvimTreeWinSeparator", "NvimTreeNormalFloatBorder",
    "AvanteSidebarWinSeparator",
    "LazyBorder",
  }) do
    hl(g, { fg = BLUE, bg = "NONE" })
  end

  for _, g in ipairs({
    "TelescopeNormal", "TelescopePromptNormal", "TelescopeResultsNormal", "TelescopePreviewNormal",
    "NoicePopup", "NoiceCmdlinePopup", "NoicePopupmenu", "NoiceMini",
    "WhichKeyFloat", "WhichKeyNormal",
    "NotifyBackground",
    "NotifyINFOBody", "NotifyWARNBody", "NotifyERRORBody", "NotifyDEBUGBody", "NotifyTRACEBody",
    "NotifyINFOTitle", "NotifyWARNTitle", "NotifyERRORTitle", "NotifyDEBUGTitle", "NotifyTRACETitle",
    "LazyNormal", "LazyBackdrop", "LazyButton", "LazyButtonActive", "LazyH1", "LazySpecial",
    "NvimTreeNormal", "NvimTreeNormalNC", "NvimTreeNormalFloat", "NvimTreeEndOfBuffer",
    "AvanteSidebarNormal",
  }) do
    hl(g, { bg = "NONE" })
  end

  hl("FloatTitle", { fg = BLUE, bg = "NONE" })

  hl("Visual", { bg = BLUE })
  hl("VisualNOS", { bg = BLUE })
end

return M
