--!strict

local SubsystemSensor = types.SubsystemSensor
local SensorTrace = types.SensorTrace
local Transform = types.Transform


local function spawnTemplate(template:Template, x:number, y:number, rotation:number?): number
    local entity = world.spawn()

    local transform = construct(Transform, {})
    transform.translation.x = x;
    transform.translation.y = y;
    if rotation then
        transform.rotation.z = rotation
    end
    world.insert_component(entity, types.Transform, transform)

    if template.sensor then
        world.insert_component(entity, types.SubsystemSensor, template.sensor)
    end
    
    if template.sensor_trace then
        world.insert_component(entity, types.SensorTrace, template.sensor_trace)  
    end

    return entity
end

type SubsystemSensor = {
    range: number,
    noise_floor: number,
}

type SensorTrace = {
    biological: number,
    electrical: number,
    gravitational: number,
    size_x: number,
    size_y: number,
}

export type Template = {
    name:string,
    sensor_trace:SensorTrace?,
    sensor:SubsystemSensor?,
}


return {spawnTemplate = spawnTemplate}