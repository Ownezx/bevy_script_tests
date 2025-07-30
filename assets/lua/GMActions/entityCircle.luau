local Template = require("./../library/Template")
local simple = require("./../templates/FirstTemplates")

local gm_action_name = "entityCircle"
local SPACING = 100

function on_gm_action(name:string, template:string, x:number, y:number, angle:number, size:number)
    if name == gm_action_name then
        local circumference = 2 * math.pi * size
        local num_ships = math.max(1, math.floor(circumference / SPACING))

        for i = 1, num_ships do
            local local_angle = (2 * math.pi / num_ships) * (i - 1)
            local spawn_x = x + math.cos(local_angle + angle) * size
            local spawn_y = y + math.sin(local_angle + angle) * size

            Template.spawnTemplate(simple[template], spawn_x, spawn_y, angle + local_angle - math.pi / 2)
        end
    end
end

register_gm_function(gm_action_name)
