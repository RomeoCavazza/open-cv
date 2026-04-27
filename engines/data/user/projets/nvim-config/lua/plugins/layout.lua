return {
  {
    "akinsho/toggleterm.nvim",
    version = "*",
    opts = {
      open_mapping = nil,
      direction = "horizontal",
      size = 14,
      shade_terminals = false,
      start_in_insert = true,
      insert_mappings = true,
      persist_size = true,
      float_opts = { border = "rounded", winblend = 0 },
    },
    config = function(_, opts)
      require("toggleterm").setup(opts)
    end,
  },

  {
    "lewis6991/gitsigns.nvim",
    event = "VeryLazy",
    opts = { signcolumn = true },
    config = function(_, opts)
      require("gitsigns").setup(opts)
    end,
  },

  {
    "folke/trouble.nvim",
    event = "VeryLazy",
    dependencies = { "nvim-tree/nvim-web-devicons" },
    opts = {
      focus = true,
      follow = true,
      auto_close = false,
      auto_preview = true,
      use_diagnostic_signs = true,
    },
    config = function(_, opts)
      require("trouble").setup(opts)
    end,
  },

  {
    "folke/edgy.nvim",
    event = "VeryLazy",
    opts = {
      animate = { enabled = false },

      left = {
        {
          title = "Explorer",
          ft = "NvimTree",
          size = { width = 32 },
          pinned = true,
          open = function()
            pcall(vim.cmd, "NvimTreeOpen")
          end,
        },
      },

      right = {
        {
          title = "Avante",
          ft = { "Avante", "AvanteInput" },
          size = { width = 52 },
          pinned = true,
          open = function()
            pcall(vim.cmd, "AvanteToggle")
          end,
        },
      },
    },

    config = function(_, opts)
      local ok, edgy = pcall(require, "edgy")
      if not ok then return end

      local ok2, err = pcall(edgy.setup, opts)
      if not ok2 then
        vim.schedule(function()
          vim.notify("edgy.nvim setup error: " .. tostring(err), vim.log.levels.ERROR)
        end)
        return
      end

      vim.g.cursor_mode_enabled = true
      vim.api.nvim_create_user_command("CursorMode", function()
        local state = vim.g.cursor_mode_enabled
        if state == nil then state = true end
        state = not state
        vim.g.cursor_mode_enabled = state

        if state then
          pcall(vim.cmd, "EdgyEnable")
          pcall(vim.cmd, "NvimTreeOpen")
        else
          pcall(vim.cmd, "EdgyDisable")
        end
      end, {})
    end,
  },
}
