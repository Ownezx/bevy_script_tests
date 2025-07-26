--!strict
local Types = require("./../library/Template")
type Template = Types.Template

local simple: { [string]: Template } = 
{
    sensor = 
    {
        name = "sensor",
        map_icon = "destroyer",
        sensor = {noise_floor = 1, range = 10000},
        sensor_trace = {size_x = 10, size_y = 10, biological = 1, gravitational = 1, electrical = 1}
    },
    sensor_trace = 
    {
        name = "trace",
        map_icon = "corvette",
        sensor_trace = {size_x = 10, size_y = 10, biological = 1, gravitational = 1, electrical = 1}
    }
}

return simple

