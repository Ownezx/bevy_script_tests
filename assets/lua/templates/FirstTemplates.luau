--!strict
local Types = require("./../library/Template")
type Template = Types.Template

local simple: { [string]: Template } = 
{
    destroyer = 
    {
        name = "destroyer",
        map_icon = "destroyer",
        sensor = {noise_floor = 1, range = 10000},
        sensor_trace = {size_x = 10, size_y = 10, biological = 1, gravitational = 1, electrical = 1},
        fixed_rotation = false,
    },
    corvette = 
    {
        name = "corvette",
        map_icon = "corvette",
        sensor_trace = {size_x = 10, size_y = 10, biological = 1, gravitational = 1, electrical = 1},
        fixed_rotation = false,
    },
    cruiser = 
    {
        name = "cruiser",
        map_icon = "cruiser",
        fixed_rotation = false,
    },
    frigate = 
    {
        name = "frigate",
        map_icon = "frigate",
        fixed_rotation = false,
    },
    mine = 
    {
        name = "mine",
        map_icon = "mine",
        fixed_rotation = true,
    }
}

return simple

