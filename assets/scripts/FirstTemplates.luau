--!strict
local Types = require("scripts.Template")
type Template = Types.Template

local simple: { [string]: Template } = 
{
    test1 = 
    {
        name = "test1"
    },
    test2 = 
    {
        name = "test2"
    }
}

return simple

