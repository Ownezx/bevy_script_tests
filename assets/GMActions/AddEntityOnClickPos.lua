local SubsystemSensor = types.SubsystemSensor
local SensorTrace = types.SensorTrace
local Transform = types.Transform


function on_click(x, y)
    print("Lua click position : {" .. x .. ", " .. y .. "}")

    local sensor = construct(SubsystemSensor, { range = 50.0, noise_floor = 1 })
    local sensor_trace = construct(SensorTrace, {
        biological = 1,
        electrical = 1,
        gravitational = 1,
        size_x = 1,
        size_y = 1,
    })
    local transform = construct(Transform, {})
    transform.translation.x = x;
    transform.translation.y = y;
    local entity = world.spawn()
    world.insert_component(entity, SubsystemSensor, sensor)
    world.insert_component(entity, Transform, transform)
    world.insert_component(entity, SensorTrace, sensor_trace)
    print("Entity with component value: " .. world.get_component(entity, SubsystemSensor).range)
    print("Entity at position: " .. transform.translation.x .. ", " .. transform.translation.y)
end
