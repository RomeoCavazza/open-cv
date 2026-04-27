local specs = {}

local function extend(t)
  for _, s in ipairs(t) do
    specs[#specs + 1] = s
  end
end

extend(require("plugins.pretty"))
extend(require("plugins.dashboard"))
extend(require("plugins.nav"))
extend(require("plugins.layout"))
extend(require("plugins.lsp"))
extend(require("plugins.git"))
extend(require("plugins.ai"))
extend(require("plugins.shellgeist"))

return specs
