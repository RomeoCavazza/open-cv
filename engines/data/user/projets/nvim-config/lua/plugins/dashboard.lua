return {
  {
    "goolord/alpha-nvim",
    event = "VimEnter",
    dependencies = { "nvim-tree/nvim-web-devicons" },
    config = function()
      vim.api.nvim_create_user_command("SGDashboardToggle", function()
        if vim.bo.filetype == "alpha" then
          vim.cmd("bd")
          return
        end
        vim.cmd("Alpha")
      end, {})

      vim.api.nvim_create_user_command("SGOpenDir", function()
        local ok = pcall(require, "telescope")
        if not ok then
          vim.notify("Telescope not available", vim.log.levels.ERROR)
          return
        end

        local pickers = require("telescope.pickers")
        local finders = require("telescope.finders")
        local conf = require("telescope.config").values
        local actions = require("telescope.actions")
        local action_state = require("telescope.actions.state")

        local cmd
        if vim.fn.executable("fd") == 1 then
          cmd = { "fd", "--type", "d", "--hidden", "--follow", "--exclude", ".git", ".", vim.fn.getcwd() }
        else
          cmd = { "find", vim.fn.getcwd(), "-type", "d", "-not", "-path", "*/.git/*" }
        end

        pickers.new({}, {
          prompt_title = "Open Directory",
          finder = finders.new_oneshot_job(cmd, {}),
          sorter = conf.generic_sorter({}),
          attach_mappings = function(prompt_bufnr, map)
            local function select_dir()
              local selection = action_state.get_selected_entry()
              actions.close(prompt_bufnr, true)
              if not selection or not selection[1] then return end
              local dir = selection[1]

              vim.cmd("cd " .. vim.fn.fnameescape(dir))
              pcall(vim.cmd, "NvimTreeOpen")
              pcall(vim.cmd, "NvimTreeFocus")
              pcall(function()
                require("telescope.builtin").find_files({ cwd = dir })
              end)
            end

            map("i", "<CR>", select_dir)
            map("n", "<CR>", select_dir)
            return true
          end,
        }):find()
      end, {})

      local alpha = require("alpha")
      local dashboard = require("alpha.themes.dashboard")

      dashboard.section.header.val = {
        "███╗   ██╗███████╗ ██████╗ ██╗   ██╗██╗███╗   ███╗",
        "████╗  ██║██╔════╝██╔═══██╗██║   ██║██║████╗ ████║",
        "██╔██╗ ██║█████╗  ██║   ██║██║   ██║██║██╔████╔██║",
        "██║╚██╗██║██╔══╝  ██║   ██║╚██╗ ██╔╝██║██║╚██╔╝██║",
        "██║ ╚████║███████╗╚██████╔╝ ╚████╔╝ ██║██║ ╚═╝ ██║",
        "╚═╝  ╚═══╝╚══════╝ ╚═════╝   ╚═══╝  ╚═╝╚═╝     ╚═╝",
        "",
      }

      dashboard.section.buttons.val = {
        dashboard.button("n", "󰎚  New file", "<cmd>ene <BAR> startinsert <cr>"),
        dashboard.button("o", "  Open Directory", "<cmd>SGOpenDir<cr>"),
        dashboard.button("f", "󰈞  Find file", "<cmd>Telescope find_files<cr>"),
        dashboard.button("g", "󰊄  Live grep", "<cmd>Telescope live_grep<cr>"),
        dashboard.button("e", "󰙅  Explorer", "<cmd>NvimTreeToggle<cr>"),
        dashboard.button("t", "󰆍  Open terminal", "<cmd>ToggleTerm<cr>"),
        dashboard.button("s", "󰚩  ShellGeist", "<cmd>SGSidebar<cr>"),
        dashboard.button("q", "󰅚  Quit", "<cmd>qa<cr>"),
      }

      alpha.setup(dashboard.config)

      vim.api.nvim_create_autocmd("FileType", {
        pattern = "alpha",
        callback = function()
          vim.keymap.set("n", "h", "<cmd>SGDashboardToggle<cr>", { buffer = true, silent = true, desc = "Close dashboard" })
        end,
      })
    end,
  },
}