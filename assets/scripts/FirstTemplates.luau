--!strict
local Types = require("./../scripts/Template")
type Template = Types.Template

local simple: { [string]: Template } = 
{
    sensor = 
    {
        name = "sensor",
        sensor = {noise_floor = 1, range = 10000},
        sensor_trace = {size_x = 10, size_y = 10, biological = 1, gravitational = 1, electrical = 1}
    },
    sensor_trace = 
    {
        name = "trace",
        sensor_trace = {size_x = 10, size_y = 10, biological = 1, gravitational = 1, electrical = 1}
    }
}

return simple

