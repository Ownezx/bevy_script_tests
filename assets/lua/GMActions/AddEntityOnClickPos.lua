local Template = require("./../library/Template")
local waarg = require("./../templates/FirstTemplates")

local is_first = true;

function on_click(x, y)
    if is_first then
        Template.spawnTemplate(waarg.cruiser,x,y)
        print("Entity ".. waarg.cruiser.name .." at position: " .. x .. ", " .. y)
        is_first = false
    else
        Template.spawnTemplate(waarg.corvette,x,y)
        print("Entity ".. waarg.corvette.name .." at position: " .. x .. ", " .. y)
    end
end


function on_script_reloaded(value)
    is_first = value
end

function on_script_unloaded()
    return is_first
end