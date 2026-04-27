return {
  { "williamboman/mason.nvim", config = true },

  {
    "williamboman/mason-lspconfig.nvim",
    dependencies = { "williamboman/mason.nvim" },
    opts = {
      ensure_installed = {
        "rust_analyzer",
        "ts_ls",
        "eslint",
        "html",
        "cssls",
        "jsonls",
        "tailwindcss",
      },
    },
  },

  {
    "neovim/nvim-lspconfig",
    dependencies = { "williamboman/mason-lspconfig.nvim", "hrsh7th/cmp-nvim-lsp" },
    config = function()
      local capabilities = require("cmp_nvim_lsp").default_capabilities()

      local servers = {
        rust_analyzer = {},
        ts_ls = {},
        eslint = {},
        html = {},
        cssls = {},
        jsonls = {},
        tailwindcss = {},
      }

      local nix_lls = "/etc/profiles/per-user/tco/bin/lua-language-server"
      local lls = nil
      if vim.fn.executable(nix_lls) == 1 then
        lls = nix_lls
      else
        local path_lls = vim.fn.exepath("lua-language-server")
        if path_lls ~= nil and path_lls ~= "" and vim.fn.executable(path_lls) == 1 then
          lls = path_lls
        end
      end

      if lls then
        servers.lua_ls = {
          cmd = { lls },
          settings = {
            Lua = {
              diagnostics = { globals = { "vim" } },
              workspace = { checkThirdParty = false },
              telemetry = { enable = false },
            },
          },
        }
      end

      for name, cfg in pairs(servers) do
        cfg.capabilities = capabilities
        vim.lsp.config(name, cfg)
        vim.lsp.enable(name)
      end

      vim.api.nvim_create_autocmd("LspAttach", {
        callback = function(args)
          local b = args.buf
          local mapb = function(mode, lhs, rhs, desc)
            vim.keymap.set(mode, lhs, rhs, { buffer = b, silent = true, desc = desc })
          end
          mapb("n", "gd", vim.lsp.buf.definition, "Go to definition")
          mapb("n", "gr", vim.lsp.buf.references, "References")
          mapb("n", "K", vim.lsp.buf.hover, "Hover")
          mapb("n", "<F2>", vim.lsp.buf.rename, "Rename")
          mapb("n", "<leader>ca", vim.lsp.buf.code_action, "Code action")
          mapb("n", "<leader>f", function() vim.lsp.buf.format({ async = true }) end, "Format")
        end,
      })
    end,
  },

  {
    "hrsh7th/nvim-cmp",
    dependencies = { "hrsh7th/cmp-nvim-lsp", "L3MON4D3/LuaSnip", "saadparwaiz1/cmp_luasnip" },
    config = function()
      local cmp = require("cmp")
      local luasnip = require("luasnip")
      cmp.setup({
        snippet = { expand = function(args) luasnip.lsp_expand(args.body) end },
        mapping = cmp.mapping.preset.insert({
          ["<CR>"] = cmp.mapping.confirm({ select = true }),
          ["<Tab>"] = cmp.mapping.select_next_item(),
          ["<S-Tab>"] = cmp.mapping.select_prev_item(),
        }),
        sources = { { name = "nvim_lsp" }, { name = "luasnip" } },
      })
    end,
  },

  { "hrsh7th/cmp-nvim-lsp" },
  { "L3MON4D3/LuaSnip" },
  { "saadparwaiz1/cmp_luasnip" },

  { "mrcjkb/rustaceanvim", version = "^5", ft = { "rust" } },
}
