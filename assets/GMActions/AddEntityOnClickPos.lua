local Template = require("./../scripts/Template")
local waarg = require("./../scripts/FirstTemplates")

local is_first = true;

function on_click(x, y)
    if is_first then
        Template.spawnTemplate(waarg.sensor,x,y)
        print("Entity ".. waarg.sensor.name .." at position: " .. x .. ", " .. y)
        is_first = false
    else
        Template.spawnTemplate(waarg.sensor_trace,x,y)
        print("Entity ".. waarg.sensor_trace.name .." at position: " .. x .. ", " .. y)
    end
end


function on_script_reloaded(value)
    is_first = value
end

function on_script_unloaded()
    return is_first
end