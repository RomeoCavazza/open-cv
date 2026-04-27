return {
  { "MunifTanjim/nui.nvim" },

  {
    "olimorris/codecompanion.nvim",
    event = "VeryLazy",
    dependencies = {
      "nvim-lua/plenary.nvim",
      "nvim-treesitter/nvim-treesitter",
      "hrsh7th/nvim-cmp",
    },
    opts = {
      adapters = {
        ollama_coder = function()
          return require("codecompanion.adapters").extend("ollama", {
            name = "ollama_coder",
            schema = {
              model = { default = "deepseek-coder-v2:16b-lite-instruct-q4_K_M" },
              num_ctx = { default = 32768 },
            },
          })
        end,
        ollama_reasoning = function()
          return require("codecompanion.adapters").extend("ollama", {
            name = "ollama_reasoning",
            schema = {
              model = { default = "deepseek-r1:14b" },
              num_ctx = { default = 16384 },
            },
          })
        end,
      },
      strategies = {
        chat = {
          adapter = "ollama_coder",
          keymaps = {
            send = { modes = { n = "<CR>", i = "<C-CR>" } },
          },
        },
        inline = { adapter = "ollama_coder" },
        agent = { adapter = "ollama_reasoning" },
      },
      display = {
        chat = {
          window = { layout = "vertical", width = 0.4 },
        },
      },
    },
    config = function(_, opts)
      require("codecompanion").setup(opts)
      vim.keymap.set({ "n", "v" }, "<leader>cc", "<cmd>CodeCompanionChat Toggle<cr>", { desc = "AI: Toggle Chat" })
      vim.keymap.set({ "n", "v" }, "<leader>ca", "<cmd>CodeCompanionActions<cr>", { desc = "AI: Actions" })
      vim.keymap.set("v", "ga", "<cmd>CodeCompanionChat Add<cr>", { desc = "AI: Add Selection to Chat" })
    end,
  },
}
