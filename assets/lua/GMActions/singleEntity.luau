local Template = require("./../library/Template")
local simple = require("./../templates/FirstTemplates")

local gm_action_name = "singleEntity"

function on_gm_action(name, template:string, x:number, y:number, angle:number)
    if name == gm_action_name then
        Template.spawnTemplate(simple[template], x, y, angle)
    end
end

register_gm_function(gm_action_name);
