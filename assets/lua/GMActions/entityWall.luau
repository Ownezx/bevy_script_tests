local Template = require("./../library/Template")
local simple = require("./../templates/FirstTemplates")

local gm_action_name = "entityWall"
local WALL_WIDTH = 5
local WALL_HEIGHT = 2
local SPACING = 100

function on_gm_action(name:string, template:string, x:number, y:number)
    if name == gm_action_name then
        local base = simple[template]
        if not base then return end

        local total_width = (WALL_WIDTH - 1) * SPACING
        local total_height = (WALL_HEIGHT - 1) * SPACING
        local start_x = x - total_width / 2
        local start_y = y - total_height / 2

        for row = 0, WALL_HEIGHT - 1 do
            for col = 0, WALL_WIDTH - 1 do
                local spawn_x = start_x + col * SPACING
                local spawn_y = start_y + row * SPACING

                Template.spawnTemplate(base, spawn_x, spawn_y)
            end
        end
    end
end

register_gm_function(gm_action_name)
