local Template = require("./../library/Template")
local simple = require("./../templates/FirstTemplates")

local gm_action_name = "entityWall"
local SPACING = 100
local WALL_HEIGHT = 2

function on_gm_action(name:string, template:string, x:number, y:number, angle:number, size:number)
    if name == gm_action_name then
        local base = simple[template]
        if not base then return end

        local wall_width = math.max(1, math.floor(size * 2 / SPACING))
        local wall_height = WALL_HEIGHT

        -- Calculate total width and height in world units
        local total_width = (wall_width - 1) * SPACING
        local total_height = (wall_height - 1) * SPACING

        -- Offsets to center the wall on the cursor
        local offset_x = total_width / 2
        local offset_y = total_height / 2

        local cos_a = math.cos(angle)
        local sin_a = math.sin(angle)

        for row = 0, wall_height - 1 do
            for col = 0, wall_width - 1 do
                -- local position before rotation, centered on cursor
                local local_x = col * SPACING - offset_x
                local local_y = row * SPACING - offset_y

                -- Apply rotation
                local rotated_x = local_x * cos_a - local_y * sin_a
                local rotated_y = local_x * sin_a + local_y * cos_a

                local spawn_x = x + rotated_x
                local spawn_y = y + rotated_y

                Template.spawnTemplate(base, spawn_x, spawn_y, angle)
            end
        end
    end
end

register_gm_function(gm_action_name)
