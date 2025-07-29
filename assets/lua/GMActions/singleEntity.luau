local Template = require("./../library/Template")
local simple = require("./../templates/FirstTemplates")

local gm_action_name = "singleEntity"

function on_gm_action(name, template, x, y)
    if name == gm_action_name then
        Template.spawnTemplate(simple[template], x, y)
        print("Spawning entity " .. simple[template].name .. " at position: " .. x .. ", " .. y)
    end
end

register_gm_function(gm_action_name);
