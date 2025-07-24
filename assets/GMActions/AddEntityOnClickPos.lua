local Template = require("scripts.Template")
local waarg = require("scripts.FirstTemplates")



function on_click(x, y)
    Template.spawnTemplate(waarg.test1,x,y)
    print("Entity".. waarg.test1.name .." at position: " .. x .. ", " .. y)
end

