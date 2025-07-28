local Template = require("./../library/Template")
local simple = require("./../templates/FirstTemplates")

local is_first = true;

function on_click(x, y)
    if is_first then
        Template.spawnTemplate(simple.cruiser,x,y)
        print("Entity ".. simple.cruiser.name .." at position: " .. x .. ", " .. y)
        is_first = false
    else
        Template.spawnTemplate(simple.corvette,x,y)
        print("Entity ".. simple.corvette.name .." at position: " .. x .. ", " .. y)
    end
end


function on_script_reloaded(value)
    is_first = value
end

function on_script_unloaded()
    return is_first
end