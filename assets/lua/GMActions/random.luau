local Template = require("./../library/Template")
local simple = require("./../templates/FirstTemplates")

local gm_action_name = "random"
local SPACING = 100

function on_gm_action(name:string, template:string, x:number, y:number, angle:number, size:number)
    if name == gm_action_name then
        local placed = {}
        local radius = size

        local max_ships = math.floor(4 * radius * radius / (SPACING * SPACING))
        local max_attempts = max_ships 
        local attempts = 0

        while #placed < max_ships and attempts < max_attempts do
            local local_angle = math.random() * 2 * math.pi
            local dist = math.sqrt(math.random()) * radius

            local spawn_x = x + math.cos(local_angle + angle) * dist
            local spawn_y = y + math.sin(local_angle + angle) * dist

            local too_close = false
            for _, pos in ipairs(placed) do
                local dx = spawn_x - pos[1]
                local dy = spawn_y - pos[2]
                if dx * dx + dy * dy < SPACING * SPACING then
                    too_close = true
                    break
                end
            end

            if not too_close then
                table.insert(placed, {spawn_x, spawn_y})
                Template.spawnTemplate(simple[template], spawn_x, spawn_y, math.random() * 2 * math.pi)
            end

            attempts = attempts + 1
        end
    end
end

register_gm_function(gm_action_name)
