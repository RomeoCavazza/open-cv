return {
  {
    "folke/tokyonight.nvim",
    priority = 1000,
    lazy = false,
    opts = {
      transparent = true,
      styles = { sidebars = "transparent", floats = "transparent" },
    },
    config = function(_, opts)
      require("tokyonight").setup(opts)
      vim.cmd.colorscheme("tokyonight")
    end,
  },

  {
    "folke/which-key.nvim",
    event = "VeryLazy",
    opts = {
      preset = "modern",
      delay = 250,
      win = {
        border = "rounded",
        padding = { 1, 1 },
        wo = { winblend = 0, winhighlight = "Normal:NormalFloat,FloatBorder:FloatBorder" },
      },
    },
    config = function(_, opts)
      local wk = require("which-key")
      wk.setup(opts)
      wk.add({
        { "<leader>a", group = "AI" },
        { "<leader>aa", desc = "Avante Ask" },
        { "<leader>ae", desc = "Avante Edit" },
        { "<leader>ad", desc = "Avante Toggle" },
        { "<leader>e", desc = "Explorer Toggle" },
        { "<leader>b", group = "Buffers" },
        { "<leader>bd", desc = "Close buffer" },
      })
    end,
  },

  {
    "folke/noice.nvim",
    event = "VeryLazy",
    dependencies = { "MunifTanjim/nui.nvim", "rcarriga/nvim-notify" },
    opts = {
      presets = { bottom_search = true, command_palette = true, long_message_to_split = true },
      views = {
        cmdline_popup = {
          border = { style = "rounded" },
          win_options = { winblend = 0, winhighlight = "Normal:NormalFloat,FloatBorder:FloatBorder" },
        },
        popup = {
          border = { style = "rounded" },
          win_options = { winblend = 0, winhighlight = "Normal:NormalFloat,FloatBorder:FloatBorder" },
        },
        popupmenu = {
          border = { style = "rounded" },
          win_options = { winblend = 0, winhighlight = "Normal:NormalFloat,FloatBorder:FloatBorder" },
        },
      },
    },
  },

  {
    "rcarriga/nvim-notify",
    event = "VeryLazy",
    opts = {
      stages = "fade",
      background_colour = "#000000",
      timeout = 2500,
    },
    config = function(_, opts)
      opts.on_open = function(win)
        pcall(vim.api.nvim_win_set_option, win, "winblend", 0)
        pcall(vim.api.nvim_win_set_option, win, "winhighlight", "Normal:NormalFloat,FloatBorder:FloatBorder")
        local buf = vim.api.nvim_win_get_buf(win)
        local ok, cc = pcall(require, "core.click_copy")
        if ok then cc.attach(buf) end
      end

      local notify = require("notify")
      notify.setup(opts)
      vim.notify = notify
    end,
  },

  {
    "nvim-lualine/lualine.nvim",
    event = "VeryLazy",
    dependencies = { "nvim-tree/nvim-web-devicons" },
    config = function()
      local status = require("core.status")
      status.start()

      require("lualine").setup({
        options = {
          theme = "auto",
          icons_enabled = true,
          section_separators = "",
          component_separators = "",
          globalstatus = true,
          disabled_filetypes = {
            statusline = { "alpha", "lazy", "mason", "lspinfo", "notify", "noice", "NvimTree" },
            winbar = { "alpha", "lazy", "mason", "lspinfo", "notify", "noice", "NvimTree" },
          },
        },
        sections = {
          lualine_a = { "mode" },
          lualine_b = {},
          lualine_c = { status.file, status.modified },
          lualine_x = { status.penguin, status.ping },
          lualine_y = { status.percent },
          lualine_z = { status.time },
        },
      })
    end,
  },

  {
    "akinsho/bufferline.nvim",
    event = "VeryLazy",
    dependencies = { "nvim-tree/nvim-web-devicons" },
    opts = {
      options = {
        diagnostics = "nvim_lsp",
        separator_style = "slant",
        show_close_icon = false,
        show_buffer_close_icons = false,
      },
    },
    config = function(_, opts)
      require("bufferline").setup(opts)
      vim.keymap.set("n", "<S-h>", "<cmd>BufferLineCyclePrev<cr>", { silent = true })
      vim.keymap.set("n", "<S-l>", "<cmd>BufferLineCycleNext<cr>", { silent = true })
      vim.keymap.set("n", "<leader>bd", "<cmd>bdelete<cr>", { silent = true, desc = "Close buffer" })
    end,
  },

  {
    "lukas-reineke/indent-blankline.nvim",
    event = "VeryLazy",
    main = "ibl",
    config = function()
      require("core.ibl").setup()
    end,
  },
}
