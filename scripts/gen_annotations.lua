

---@class neovim.Version
---@field major number
---@field minor number
---@field patch number
---@field prerelease boolean
---@field api_level number
---@field api_compatible number
---@field api_prerelease boolean
---@field build string


---@alias neovim.Parameter [string, string]

---@class neovim.Function
---@field name string
---@field method boolean
---@field parameters neovim.Parameter[]
---@field since number
---@field deprecated_since? number
---@field return_type string


---@class neovim.UIEvent
---@field name string
---@field parameters neovim.Parameter[]
---@field since number


---@class neovim.ApiInfo
---@field version neovim.Version
---@field functions neovim.Function[]
---@field ui_events neovim.UIEvent[]
---@field ui_options string[]
---@field types {[string]: {id: number, prefix: string}}
---@field error_types {[string]: {id: number}}


local function main()
    ---@type neovim.ApiInfo
    local api_info = vim.fn.api_info()
    

end


main()
