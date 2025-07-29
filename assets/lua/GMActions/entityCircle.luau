local Template = require("./../library/Template")
local simple = require("./../templates/FirstTemplates")

local gm_action_name = "entityCircle"
local NUM_SHIPS = 8
local RADIUS = 200

function on_gm_action(name:string, template:string, x:number, y:number)
    if name == gm_action_name then
        for i = 1, NUM_SHIPS do
            local angle = (2 * math.pi / NUM_SHIPS) * (i - 1)
            local spawn_x = x + math.cos(angle) * RADIUS
            local spawn_y = y + math.sin(angle) * RADIUS

            Template.spawnTemplate(simple[template], spawn_x, spawn_y, angle - math.pi/2)
        end
    end
end

register_gm_function(gm_action_name)
